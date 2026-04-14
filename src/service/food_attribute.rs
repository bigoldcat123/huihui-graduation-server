use crate::{model::output::FoodAttribute, service::error::ServiceError, source};

pub async fn get_food_attribute(food_id: i32) -> Result<FoodAttribute, ServiceError> {
    let attr = source::food_attribute::get_food_attribute(food_id).await?;
    Ok(attr)
}

pub async fn create_food_attribute(attr: &FoodAttribute) -> Result<FoodAttribute, ServiceError> {
    let created = source::food_attribute::create_food_attribute(attr).await?;
    Ok(created)
}

pub async fn update_food_attribute(
    food_id: i32,
    attr: &FoodAttribute,
) -> Result<FoodAttribute, ServiceError> {
    let updated = source::food_attribute::update_food_attribute(food_id, attr).await?;
    Ok(updated)
}

pub async fn upsert_food_attribute(attr: &FoodAttribute) -> Result<FoodAttribute, ServiceError> {
    let upserted = source::food_attribute::upsert_food_attribute(attr).await?;
    Ok(upserted)
}

pub async fn delete_food_attribute(food_id: i32) -> Result<(), ServiceError> {
    source::food_attribute::delete_food_attribute(food_id).await?;
    Ok(())
}
