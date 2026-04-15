use crate::{db, model::raw::FoodCommentRow};

pub async fn get_food_comments_by_food_id(
    food_id: i32,
    current_user_id: i32,
) -> Result<Vec<FoodCommentRow>, sqlx::Error> {
    let comments: Vec<FoodCommentRow> = sqlx::query_as(
        r#"
        SELECT
            fc.id,
            fc.food_id,
            fc.user_id,
            fc.content,
            fc.create_time,
            COALESCE(t.thumb_count, 0)::BIGINT AS thumb_count,
            EXISTS(
                SELECT 1 FROM food_comment_thumb fct2
                WHERE fct2.food_comment_id = fc.id AND fct2.user_id = $2
            ) AS thumbed
        FROM food_comment fc
        LEFT JOIN (
            SELECT food_comment_id, COUNT(*)::BIGINT AS thumb_count
            FROM food_comment_thumb
            GROUP BY food_comment_id
        ) t ON t.food_comment_id = fc.id
        WHERE fc.food_id = $1
        ORDER BY fc.create_time DESC
        "#,
    )
    .bind(food_id)
    .bind(current_user_id)
    .fetch_all(db())
    .await?;
    Ok(comments)
}

pub async fn create_food_comment(
    food_id: i32,
    user_id: i32,
    content: &str,
) -> Result<FoodCommentRow, sqlx::Error> {
    let comment: FoodCommentRow = sqlx::query_as(
        r#"
        INSERT INTO food_comment (food_id, user_id, content)
        VALUES ($1, $2, $3)
        RETURNING id, food_id, user_id, content, create_time, 0::BIGINT AS thumb_count, FALSE AS thumbed
        "#,
    )
    .bind(food_id)
    .bind(user_id)
    .bind(content)
    .fetch_one(db())
    .await?;
    Ok(comment)
}

pub async fn toggle_thumb(user_id: i32, food_comment_id: i32) -> Result<bool, sqlx::Error> {
    let deleted: Option<(i32,)> = sqlx::query_as(
        r#"
        DELETE FROM food_comment_thumb
        WHERE user_id = $1 AND food_comment_id = $2
        RETURNING user_id
        "#,
    )
    .bind(user_id)
    .bind(food_comment_id)
    .fetch_optional(db())
    .await?;

    if deleted.is_some() {
        return Ok(false);
    }

    sqlx::query(
        r#"
        INSERT INTO food_comment_thumb (user_id, food_comment_id)
        VALUES ($1, $2)
        "#,
    )
    .bind(user_id)
    .bind(food_comment_id)
    .execute(db())
    .await?;

    Ok(true)
}
