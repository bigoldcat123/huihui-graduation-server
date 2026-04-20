use crate::{
    model::{input::{CreateExerciseTypeInput, UpdateExerciseTypeInput}, output::ExerciseTypeOutput},
    service::error::ServiceError,
    source,
};

pub async fn list() -> Result<Vec<ExerciseTypeOutput>, ServiceError> {
    let records = source::exercise_type::list().await?;
    Ok(records.into_iter().map(|r| r.into()).collect())
}

pub async fn get_by_id(id: i32) -> Result<ExerciseTypeOutput, ServiceError> {
    let record = source::exercise_type::get_by_id(id).await?;
    Ok(record.into())
}

pub async fn create(input: CreateExerciseTypeInput) -> Result<ExerciseTypeOutput, ServiceError> {
    let record = source::exercise_type::create(&input.name, input.met_value).await?;
    Ok(record.into())
}

pub async fn update(input: UpdateExerciseTypeInput) -> Result<ExerciseTypeOutput, ServiceError> {
    let record = source::exercise_type::update(input.id, &input.name, input.met_value).await?;
    Ok(record.into())
}

pub async fn delete(id: i32) -> Result<(), ServiceError> {
    source::exercise_type::delete(id).await?;
    Ok(())
}
