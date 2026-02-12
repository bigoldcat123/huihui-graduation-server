use std::collections::HashMap;

use crate::{model::{input::{Reaction, RecommendationReactionInput, SuggestionInput}, raw::Tag}, service::error::ServiceError, source::{self, operation}};
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
    let foods = source::food::list_foods().await?;
    Ok(foods)
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
    // println!("{:?}",ipt.food_ids);
    // println!("{:?}",unselected_food.iter().map(|x| x.name.as_str()).collect::<Vec<_>>());

    if selected_food.is_empty(){
        return Ok(unselected_food[unselected_food.len() - 4..].iter().cloned().collect())
    }
    let mut selected_foodtags = HashMap::new();
    for food in &selected_food {
        let tags = source::tag::list_food_tags(food.id).await?;
        selected_foodtags.insert(food.id, tags);
    }
    let user_profile = cal_user_profile(selected_foodtags);
    let mut unselected_foodtags = HashMap::new();
    for food in &unselected_food {
        let tags = source::tag::list_food_tags(food.id).await?;
        unselected_foodtags.insert(food.id, tags);
    }
    let mut food_factor = HashMap::new();
    for (f_id, tags) in unselected_foodtags {
        food_factor.insert(f_id, test_food(&user_profile, &tags));
    }
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

pub fn cal_user_profile(food_tags: HashMap<i32, Vec<Tag>>) -> HashMap<i32, f32> {
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
    dot / (user_m * food_m)
}
