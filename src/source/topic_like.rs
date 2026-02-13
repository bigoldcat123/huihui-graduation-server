use crate::db;

pub async fn like_topic(user_id: i32, topic_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO topic_like (user_id, topic_id)
        VALUES ($1, $2)
        ON CONFLICT (user_id, topic_id) DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(topic_id)
    .execute(db())
    .await?;
    Ok(())
}

pub async fn unlike_topic(user_id: i32, topic_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM topic_like
        WHERE user_id = $1 AND topic_id = $2
        "#,
    )
    .bind(user_id)
    .bind(topic_id)
    .execute(db())
    .await?;
    Ok(())
}
