use crate::{db, model::raw::MealRecord};

pub async fn get_today_meal_records(user_id: i32) -> Result<Vec<MealRecord>, sqlx::Error> {
    let records: Vec<MealRecord> = sqlx::query_as(
        r#"
        SELECT id, user_id, meal_type, source_type, total_calories, note, created_at, updated_at
        FROM meal_record
        WHERE user_id = $1
          AND DATE(created_at) = CURRENT_DATE
        ORDER BY meal_type
        "#,
    )
    .bind(user_id)
    .fetch_all(db())
    .await?;
    Ok(records)
}

pub async fn create_meal_record(
    user_id: i32,
    meal_type: &str,
    source_type: &str,
    total_calories: f32,
    note: Option<&str>,
) -> Result<MealRecord, sqlx::Error> {
    let record: MealRecord = sqlx::query_as(
        r#"
        INSERT INTO meal_record (user_id, meal_type, source_type, total_calories, note)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, meal_type, source_type, total_calories, note, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(meal_type)
    .bind(source_type)
    .bind(total_calories)
    .bind(note)
    .fetch_one(db())
    .await?;
    Ok(record)
}
