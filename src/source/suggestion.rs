use crate::db;

pub async fn create_suggestion(
    user_id: i32,
    content: &str,
    images_json: &str,
    suggestion_type: &str,
    food_id: Option<i32>,
    restaurant_id: Option<i32>,
) -> Result<i32, sqlx::Error> {
    let new_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO suggestion (content, images, type, food_id, restaurant_id, user_id)
        VALUES ($1, $2, $3::suggestion_type, $4, $5, $6)
        RETURNING id
        "#,
    )
    .bind(content)
    .bind(images_json)
    .bind(suggestion_type)
    .bind(food_id)
    .bind(restaurant_id)
    .bind(user_id)
    .fetch_one(db())
    .await?;
    Ok(new_id)
}
