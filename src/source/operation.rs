use crate::db;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct ReactionCountRow {
    pub like: i32,
    pub dislike: i32,
}

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

pub async fn count_like_dislike_by_user(uid: i32) -> Result<ReactionCountRow, sqlx::Error> {
    let row: ReactionCountRow = sqlx::query_as(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE name = 'like' AND weight > 0)::int AS like,
            COUNT(*) FILTER (WHERE name = 'dislike' AND weight < 0)::int AS dislike
        FROM operation
        WHERE user_id = $1
        "#,
    )
    .bind(uid)
    .fetch_one(db())
    .await?;
    Ok(row)
}
