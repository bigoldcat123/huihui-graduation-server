use crate::{
    model::{input::{CreateSuggestionInput, ReviewSuggestionInput}, output::{FoodTag, FoodWithTags, Restaurant, Suggestion}},
    service::error::ServiceError,
    source,
};

pub async fn create(user_id: i32, ipt: CreateSuggestionInput) -> Result<i32, ServiceError> {
    let images_json = serde_json::to_string(&ipt.images)?;
    let new_id = source::suggestion::create_suggestion(
        user_id,
        &ipt.content,
        &images_json,
        ipt.r#type.as_db_str(),
        ipt.food_id,
        ipt.restaurant_id,
    )
    .await?;
    Ok(new_id)
}

pub async fn list_my(user_id: i32) -> Result<Vec<Suggestion>, ServiceError> {
    let suggestions = source::suggestion::list_my_suggestions(user_id).await?;
    map_suggestions(suggestions).await
}

pub async fn list_by_page(page: Option<i64>, page_size: Option<i64>) -> Result<Vec<Suggestion>, ServiceError> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);
    let suggestions = source::suggestion::list_suggestions_by_page(page, page_size).await?;
    map_suggestions(suggestions).await
}

pub async fn review(root_user_id: i32, ipt: ReviewSuggestionInput) -> Result<(), ServiceError> {
    source::suggestion::review_suggestion(
        ipt.suggestion_id,
        root_user_id,
        ipt.status.as_db_str(),
        &ipt.review_comment,
    )
    .await?;
    Ok(())
}

async fn map_suggestions(suggestions: Vec<crate::model::raw::Suggestion>) -> Result<Vec<Suggestion>, ServiceError> {
    let mut result = Vec::with_capacity(suggestions.len());

    for item in suggestions {
        let food = match item.food_id {
            Some(food_id) => {
                let food = source::food::get_food_by_id(food_id).await?;
                let restaurant = source::restaurant::get_restaurant_by_id(food.restaurant_id).await?;
                let tags = source::tag::list_food_tags(food.id).await?;
                Some(FoodWithTags {
                    id: food.id,
                    restaurant_id: food.restaurant_id,
                    restaurant_name: restaurant.name,
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
            None => None,
        };

        let restaurant = match item.restaurant_id {
            Some(restaurant_id) => {
                let r = source::restaurant::get_restaurant_by_id(restaurant_id).await?;
                Some(Restaurant::from(r))
            }
            None => None,
        };

        result.push(Suggestion {
            id: item.id,
            content: item.content,
            images: item
                .images
                .and_then(|images| serde_json::from_str::<Vec<String>>(&images).ok()),
            r#type: item.r#type,
            status: item.status,
            food,
            restaurant,
            reviewer_id: item.reviewer_id,
            review_comment: item.review_comment,
            user_id: item.user_id,
            created_at: item.created_at.to_rfc3339(),
            reviewed_at: item.reviewed_at.map(|t| t.to_rfc3339()),
        });
    }

    Ok(result)
}
