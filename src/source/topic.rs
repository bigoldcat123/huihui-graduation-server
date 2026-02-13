use crate::{db, model::raw::Topic};

pub async fn list_topics_by_page(page: i64, page_size: i64) -> Result<Vec<Topic>, sqlx::Error> {
    let page = if page < 1 { 1 } else { page };
    let offset = (page - 1) * page_size;
    let topics: Vec<Topic> = sqlx::query_as(
        r#"
        SELECT id, user_id, title, content, images, create_at
        FROM topic
        WHERE is_top = TRUE
        ORDER BY create_at DESC, id DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(db())
    .await?;
    Ok(topics)
}
