use crate::{db, model::raw::ExerciseRecord};

pub async fn get_by_user(user_id: i32) -> Result<Vec<ExerciseRecord>, sqlx::Error> {
    let records: Vec<ExerciseRecord> = sqlx::query_as(
        r#"
        SELECT id, user_id, exercise_type_id, exercise_name_snapshot,
               met_value_snapshot, duration_minutes, body_weight_kg,
               calories_burned, occurred_at, created_at, updated_at
        FROM exercise_record
        WHERE user_id = $1
        ORDER BY occurred_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(db())
    .await?;
    Ok(records)
}

pub async fn get_today_by_user(user_id: i32) -> Result<Vec<ExerciseRecord>, sqlx::Error> {
    let records: Vec<ExerciseRecord> = sqlx::query_as(
        r#"
        SELECT id, user_id, exercise_type_id, exercise_name_snapshot,
               met_value_snapshot, duration_minutes, body_weight_kg,
               calories_burned, occurred_at, created_at, updated_at
        FROM exercise_record
        WHERE user_id = $1
          AND  occurred_at + INTERVAL '8 hours'  = CURRENT_DATE
        ORDER BY occurred_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(db())
    .await?;
    Ok(records)
}

pub async fn create(
    user_id: i32,
    exercise_type_id: i32,
    exercise_name_snapshot: &str,
    met_value_snapshot: f32,
    duration_minutes: i32,
    body_weight_kg: f32,
    calories_burned: f32,
    occurred_at: chrono::DateTime<chrono::Local>,
) -> Result<ExerciseRecord, sqlx::Error> {
    let record: ExerciseRecord = sqlx::query_as(
        r#"
        INSERT INTO exercise_record (
            user_id, exercise_type_id, exercise_name_snapshot,
            met_value_snapshot, duration_minutes, body_weight_kg,
            calories_burned, occurred_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, user_id, exercise_type_id, exercise_name_snapshot,
                  met_value_snapshot, duration_minutes, body_weight_kg,
                  calories_burned, occurred_at, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(exercise_type_id)
    .bind(exercise_name_snapshot)
    .bind(met_value_snapshot)
    .bind(duration_minutes)
    .bind(body_weight_kg)
    .bind(calories_burned)
    .bind(occurred_at)
    .fetch_one(db())
    .await?;
    Ok(record)
}
