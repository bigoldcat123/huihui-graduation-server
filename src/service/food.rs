use std::collections::HashMap;

use crate::{model::{input::{CreateFoodInput, Reaction, RecommendationReactionInput, SuggestionInput, UpdateFoodInput}, output::{FoodTag, FoodWithTags}, raw::Tag}, service::error::ServiceError, source::{self, operation}};
use rand::seq::SliceRandom;
use source::food::FoodRow;

pub async fn init_suggest() -> Result<Vec<FoodRow>, ServiceError> {
    let mut tags = source::tag::list_tags().await?;
    if tags.len() > 4 {
        tags.truncate(4);
    }
    let foods = source::food::init_suggest(tags).await?;
    Ok(foods)
}

pub async fn recommendation(_user_id: i32) -> Result<Vec<FoodRow>, ServiceError> {
    let foods = source::food::list_user_liked_foods(_user_id).await?;
    let food_tag = cal_food_tags(&foods).await?;

    //1. calculate user's food preference
    let user_profile = cal_user_profile(food_tag);
    println!("{:?}",user_profile);
    //2. calculate all food's preference
    let all_foods = source::food::list_foods().await?;
    let all_food_tag = cal_food_tags(&all_foods).await?;
    let food_factor = cal_food_factor(&user_profile, &all_food_tag);
    println!("{:?}",food_factor);
    //3. take all food above 0.5
    let mut filtered_foods = all_foods.iter().filter(|x| food_factor[&x.id] >= 0.5).cloned().collect::<Vec<_>>();
    let mut left_foods = all_foods.iter().filter(|x| food_factor[&x.id] < 0.5).cloned().collect::<Vec<_>>();
    //4. take 10 randomly
    if filtered_foods.len() < 10 {
        left_foods.shuffle(&mut rand::rng());
        while filtered_foods.len() < 10 {

            if let Some(food) = left_foods.pop() {
                filtered_foods.push(food);
            }
        }
    }
    filtered_foods.shuffle(&mut rand::rng());
    Ok(filtered_foods)
}

pub async fn list_foods_by_page(page: Option<i64>, page_size: Option<i64>) -> Result<Vec<FoodWithTags>, ServiceError> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10).max(1).min(100);
    let foods = source::food::list_foods_by_page(page, page_size).await?;
    let mut result = Vec::with_capacity(foods.len());
    for food in foods {
        let tags = source::tag::list_food_tags(food.id).await?;
        result.push(FoodWithTags {
            id: food.id,
            restaurant_id: food.restaurant_id,
            name: food.name,
            description: food.description,
            image: food.image,
            tags: tags
                .into_iter()
                .map(|t| FoodTag {
                    id: t.id,
                    name: t.name,
                    image: t.image,
                })
                .collect(),
        });
    }
    Ok(result)
}

pub async fn create_food(ipt: CreateFoodInput) -> Result<FoodWithTags, ServiceError> {
    let food = source::food::create_food(
        ipt.restaurant_id,
        &ipt.name,
        &ipt.description,
        &ipt.image,
    )
    .await?;

    if let Some(tag_ids) = ipt.tag_ids {
        for tag_id in tag_ids {
            source::food::add_food_tag(food.id, tag_id).await?;
        }
    }

    let tags = source::tag::list_food_tags(food.id).await?;
    Ok(FoodWithTags {
        id: food.id,
        restaurant_id: food.restaurant_id,
        name: food.name,
        description: food.description,
        image: food.image,
        tags: tags
            .into_iter()
            .map(|t| FoodTag {
                id: t.id,
                name: t.name,
                image: t.image,
            })
            .collect(),
    })
}

pub async fn update_food(ipt: UpdateFoodInput) -> Result<FoodWithTags, ServiceError> {
    let food = source::food::update_food(
        ipt.id,
        ipt.restaurant_id,
        &ipt.name,
        &ipt.description,
        &ipt.image,
    )
    .await?;

    source::food::clear_food_tags(food.id).await?;
    for tag_id in ipt.tag_ids {
        source::food::add_food_tag(food.id, tag_id).await?;
    }

    let tags = source::tag::list_food_tags(food.id).await?;
    Ok(FoodWithTags {
        id: food.id,
        restaurant_id: food.restaurant_id,
        name: food.name,
        description: food.description,
        image: food.image,
        tags: tags
            .into_iter()
            .map(|t| FoodTag {
                id: t.id,
                name: t.name,
                image: t.image,
            })
            .collect(),
    })
}

pub async fn save_reaction(user_id: i32, ipt: RecommendationReactionInput) -> Result<i32, ServiceError> {
    let reaction_name = match ipt.reaction {
        Reaction::Like => "like",
        Reaction::Skip => "skip",
        Reaction::Dislike => "dislike",
    };
    let weight = match ipt.reaction {
        Reaction::Like => 1.0,
        Reaction::Skip => 0.0,
        Reaction::Dislike => -1.0,
    };
    let op_id = operation::save_operation(user_id, ipt.food_id, reaction_name, weight).await?;
    Ok(op_id)
}

pub async fn consecutive_suggest(ipt:SuggestionInput,user_id:i32) -> Result<Vec<FoodRow>, ServiceError> {

    for &s_id in ipt.selected_food_ids.iter() {
        let _ = operation::save_operation(user_id, s_id, "like", 1.0).await?;
    }

    let selected_food = source::food::list_food_in_ids(&ipt.selected_food_ids).await?;
    let mut unselected_food = source::food::list_food_not_in_ids(&ipt.food_ids).await?;

    if selected_food.is_empty(){
        return Ok(unselected_food[unselected_food.len() - 4..].iter().cloned().collect())
    }
    let selected_foodtags = cal_food_tags(&selected_food).await?;
    let user_profile = cal_user_profile(selected_foodtags);

    let unselected_foodtags = cal_food_tags(&unselected_food).await?;

    let food_factor = cal_food_factor(&user_profile, &unselected_foodtags);
    unselected_food.sort_by(|a, b| {
        let a = food_factor.get(&a.id).copied().unwrap_or_default();
        let b = food_factor.get(&b.id).copied().unwrap_or_default();
        a.partial_cmp(&b).expect("float compare failed")
    });
    if unselected_food.len() < 4 {
        Ok(unselected_food)
    }else {
        let mut ans:Vec<FoodRow> = unselected_food[unselected_food.len() - 2..].iter().cloned().collect();
        ans.push(unselected_food[0].clone());
        ans.push(unselected_food[unselected_food.len() / 2].clone());
        Ok(ans)
    }
}
fn cal_food_factor(user_profile:&HashMap<i32, f32>,food_tag:&HashMap<i32,Vec<Tag>>) -> HashMap<i32,f32> {
    let mut food_factor = HashMap::new();
    for (&f_id, tags) in food_tag {
        food_factor.insert(f_id, test_food(&user_profile, &tags));
    }
    food_factor
}

async  fn cal_food_tags(foods: &Vec<FoodRow>) -> Result<HashMap<i32,Vec<Tag>>,sqlx::Error> {
    let mut foodtags = HashMap::new();
    for food in foods {
        let tags = source::tag::list_food_tags(food.id).await?;
        foodtags.insert(food.id, tags);
    }
    Ok(foodtags)
}

fn cal_user_profile(food_tags: HashMap<i32, Vec<Tag>>) -> HashMap<i32, f32> {
    let total_weight = food_tags.values().map(|x| x.len()).sum::<usize>() as f32;

    if total_weight == 0.0 {
        return HashMap::new();
    }

    let mut ans = HashMap::new();
    for (_, tags) in food_tags {
        for t in tags {
            *ans.entry(t.id).or_default() += 1.0;
        }
    }
    for (_, count) in &mut ans {
        *count /= total_weight;
    }

    ans
}

fn test_food(user_vector: &HashMap<i32, f32>, food_tags: &Vec<Tag>) -> f32 {
    let mut dot = 0.0;
    for t in food_tags {
        dot += user_vector.get(&t.id).copied().unwrap_or_default();
    }
    let user_m = user_vector.values().map(|x| x * x).sum::<f32>().sqrt();
    let food_m = (food_tags.len() as f32).sqrt();
    if dot == 0.0 || user_m == 0.0 || food_m == 0.0{
        return 0.0
    } else {
        dot / (user_m * food_m)
    }
}
