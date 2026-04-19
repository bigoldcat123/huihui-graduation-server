use crate::{
    model::output::MealRecordOutput,
    service::error::ServiceError,
    source,
};

pub async fn get_today_records(
    user_id: i32,
) -> Result<Vec<MealRecordOutput>, ServiceError> {
    let records = source::meal_record::get_today_meal_records(user_id).await?;
    Ok(records.into_iter().map(|r| r.into()).collect())
}
