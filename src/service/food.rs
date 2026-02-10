use crate::{service::error::ServiceError, source};
use source::food::FoodRow;

pub async fn init_suggest() -> Result<Vec<FoodRow>, ServiceError> {
    let mut tags = source::tag::list_tags().await?;
    if tags.len() > 4 {
        tags.truncate(4);
    }
    let foods = source::food::init_suggest(tags).await?;
    Ok(foods)
}

pub async fn consecutive_suggest(_food_ids: Vec<i32>) -> Result<Vec<FoodRow>, ServiceError> {
    let _un_selected_food = source::food::list_food_not_in_ids(_food_ids).await?;
    Ok(Vec::new())
}
