use crate::{db, model::raw::ExerciseType};

pub async fn list() -> Result<Vec<ExerciseType>, sqlx::Error> {
    let records: Vec<ExerciseType> = sqlx::query_as(
        r#"
        SELECT id, name, met_value, created_at, updated_at
        FROM exercise_type
        ORDER BY id
        "#,
    )
    .fetch_all(db())
    .await?;
    Ok(records)
}

pub async fn get_by_id(id: i32) -> Result<ExerciseType, sqlx::Error> {
    let record: ExerciseType = sqlx::query_as(
        r#"
        SELECT id, name, met_value, created_at, updated_at
        FROM exercise_type
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(db())
    .await?;
    Ok(record)
}

pub async fn create(name: &str, met_value: f32) -> Result<ExerciseType, sqlx::Error> {
    let record: ExerciseType = sqlx::query_as(
        r#"
        INSERT INTO exercise_type (name, met_value)
        VALUES ($1, $2)
        RETURNING id, name, met_value, created_at, updated_at
        "#,
    )
    .bind(name)
    .bind(met_value)
    .fetch_one(db())
    .await?;
    Ok(record)
}

pub async fn update(id: i32, name: &str, met_value: f32) -> Result<ExerciseType, sqlx::Error> {
    let record: ExerciseType = sqlx::query_as(
        r#"
        UPDATE exercise_type
        SET name = $1, met_value = $2
        WHERE id = $3
        RETURNING id, name, met_value, created_at, updated_at
        "#,
    )
    .bind(name)
    .bind(met_value)
    .bind(id)
    .fetch_one(db())
    .await?;
    Ok(record)
}

pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
    sqlx::query(r#"DELETE FROM exercise_type WHERE id = $1"#)
        .bind(id)
        .execute(db())
        .await?;
    Ok(())
}
