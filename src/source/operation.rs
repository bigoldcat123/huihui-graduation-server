use crate::db;

pub async fn save_operation(uid: i32, fid: i32, name: &str, weight: f32) -> Result<i32, sqlx::Error> {
    let new_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO operation (user_id, food_id, name, weight)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind(uid)
    .bind(fid)
    .bind(name)
    .bind(weight)
    .fetch_one(db())
    .await?;

    Ok(new_id)
}
