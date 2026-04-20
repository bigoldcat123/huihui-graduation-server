use crate::{
    model::{input::CreateExerciseRecordInput, output::ExerciseRecordOutput},
    service::error::ServiceError,
    source,
};
use chrono::{Datelike, Local, NaiveDateTime, TimeZone, Timelike};

pub async fn get_all_records(user_id: i32) -> Result<Vec<ExerciseRecordOutput>, ServiceError> {
    let records = source::exercise_record::get_by_user(user_id).await?;
    Ok(records.into_iter().map(|r| r.into()).collect())
}

pub async fn get_today_records(user_id: i32) -> Result<Vec<ExerciseRecordOutput>, ServiceError> {
    let records = source::exercise_record::get_today_by_user(user_id).await?;
    log::info!("{records:?}");
    Ok(records.into_iter().map(|r| r.into()).collect())
}

pub async fn create_record(
    user_id: i32,
    input: CreateExerciseRecordInput,
) -> Result<ExerciseRecordOutput, ServiceError> {
    let exercise_type = source::exercise_type::get_by_id(input.exercise_type_id).await?;

    let occurred_at = NaiveDateTime::parse_from_str(&input.occurred_at, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| ServiceError::InvalidInput("invalid occurred_at format".into()))?;

    let occurred_at = Local
        .with_ymd_and_hms(
            occurred_at.year(),
            occurred_at.month(),
            occurred_at.day(),
            occurred_at.hour(),
            occurred_at.minute(),
            occurred_at.second(),
        )
        .unwrap();
    let calories_burned =
        exercise_type.met_value * input.body_weight_kg * (input.duration_minutes as f32 / 60.0);

    let record = source::exercise_record::create(
        user_id,
        input.exercise_type_id,
        &exercise_type.name,
        exercise_type.met_value,
        input.duration_minutes,
        input.body_weight_kg,
        calories_burned,
        occurred_at,
    )
    .await?;

    Ok(record.into())
}
