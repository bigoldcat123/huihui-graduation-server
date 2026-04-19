use crate::{
    model::{input::{MealRecordInput, InsertMealRecordFromFoodInput}, output::MealRecordOutput},
    service::error::ServiceError,
    source,
};

pub async fn get_today_records(
    user_id: i32,
) -> Result<Vec<MealRecordOutput>, ServiceError> {
    let records = source::meal_record::get_today_meal_records(user_id).await?;
    Ok(records.into_iter().map(|r| r.into()).collect())
}

pub async fn create_record(
    user_id: i32,
    input: MealRecordInput,
) -> Result<MealRecordOutput, ServiceError> {
    let record = source::meal_record::create_meal_record(
        user_id,
        &input.meal_type,
        &input.source_type,
        input.total_calories,
        input.note.as_deref(),
    )
    .await?;
    Ok(record.into())
}

pub async fn insert_from_inner_food(
    user_id: i32,
    input: InsertMealRecordFromFoodInput,
) -> Result<MealRecordOutput, ServiceError> {
    println!("{:?}",input.food_id);
    let attr = source::food_attribute::get_food_attribute(input.food_id).await?;
    let record = source::meal_record::create_meal_record(
        user_id,
        &input.meal_type,
        "Inner",
        attr.calories as f32,
        None,
    )
    .await?;
    Ok(record.into())
}
