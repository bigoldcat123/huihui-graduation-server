use crate::{db, model::raw::UserProfile};
use chrono::NaiveDate;

pub async fn get_user_profile(user_id: i32) -> Result<Option<UserProfile>, sqlx::Error> {
    let profile: Option<UserProfile> = sqlx::query_as(
        "SELECT id, user_id, CAST(height_cm AS float8), CAST(weight_kg AS float8), birth_date, gender, created_at, updated_at FROM user_profile WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_optional(db())
    .await?;
    Ok(profile)
}

pub async fn upsert_user_profile(
    user_id: i32,
    height_cm: f64,
    weight_kg: f64,
    birth_date: Option<NaiveDate>,
    gender: &str,
) -> Result<UserProfile, sqlx::Error> {
    let profile: UserProfile = sqlx::query_as(
        r#"
        INSERT INTO user_profile (user_id, height_cm, weight_kg, birth_date, gender)
        VALUES ($1, CAST($2 AS numeric), CAST($3 AS numeric), $4, $5)
        ON CONFLICT (user_id) DO UPDATE SET
            height_cm = CAST($2 AS numeric),
            weight_kg = CAST($3 AS numeric),
            birth_date = $4,
            gender = $5,
            updated_at = now()
        RETURNING id, user_id, CAST(height_cm AS float8), CAST(weight_kg AS float8), birth_date, gender, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(height_cm)
    .bind(weight_kg)
    .bind(birth_date)
    .bind(gender)
    .fetch_one(db())
    .await?;
    Ok(profile)
}
