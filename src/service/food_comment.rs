use crate::{model::output::FoodComment, service::error::ServiceError, source};

pub async fn get_food_comments(food_id: i32, user_id: i32) -> Result<Vec<FoodComment>, ServiceError> {
    let comments = source::food_comment::get_food_comments_by_food_id(food_id, user_id).await?;
    Ok(comments.into_iter().map(FoodComment::from).collect())
}

pub async fn create_food_comment(
    food_id: i32,
    user_id: i32,
    content: &str,
) -> Result<FoodComment, ServiceError> {
    let comment = source::food_comment::create_food_comment(food_id, user_id, content).await?;
    Ok(FoodComment::from(comment))
}

pub async fn toggle_thumb(user_id: i32, food_comment_id: i32) -> Result<bool, ServiceError> {
    let thumbed = source::food_comment::toggle_thumb(user_id, food_comment_id).await?;
    Ok(thumbed)
}
