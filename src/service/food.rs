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
