use crate::{db, model::raw::{Topic, TopicWithStats}};

pub async fn list_topics_by_page(
    page: i64,
    page_size: i64,
    current_user_id: i32,
) -> Result<Vec<TopicWithStats>, sqlx::Error> {
    let page = if page < 1 { 1 } else { page };
    let offset = (page - 1) * page_size;
    let topics: Vec<TopicWithStats> = sqlx::query_as(
        r#"
        SELECT
            t.id,
            t.user_id,
            t.title,
            t.content,
            t.images,
            t.create_at,
            u.username AS user_name,
            u.email AS user_email,
            u.profile AS user_profile,
            COALESCE(c.comment_count, 0)::BIGINT AS comment_count,
            COALESCE(l.like_count, 0)::BIGINT AS like_count,
            EXISTS(
                SELECT 1 FROM topic_like tl2
                WHERE tl2.topic_id = t.id AND tl2.user_id = $3
            ) AS liked
        FROM topic t
        JOIN _user u ON u.id = t.user_id
        LEFT JOIN (
            SELECT comment_to_id, COUNT(*)::BIGINT AS comment_count
            FROM reply
            GROUP BY comment_to_id
        ) c ON c.comment_to_id = t.id
        LEFT JOIN (
            SELECT topic_id, COUNT(*)::BIGINT AS like_count
            FROM topic_like
            GROUP BY topic_id
        ) l ON l.topic_id = t.id
        WHERE t.is_top = TRUE
        ORDER BY t.create_at DESC, t.id DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size)
    .bind(offset)
    .bind(current_user_id)
    .fetch_all(db())
    .await?;
    Ok(topics)
}

pub async fn create_topic(
    user_id: i32,
    title: &str,
    content: &str,
    is_top: bool,
    images: Option<&str>,
) -> Result<Topic, sqlx::Error> {
    let topic: Topic = sqlx::query_as(
        r#"
        INSERT INTO topic (user_id, title, content, is_top, images)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, title, content, images, create_at
        "#,
    )
    .bind(user_id)
    .bind(title)
    .bind(content)
    .bind(is_top)
    .bind(images)
    .fetch_one(db())
    .await?;
    Ok(topic)
}

pub async fn create_reply(comment_id: i32, comment_to_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO reply (comment_id, comment_to_id)
        VALUES ($1, $2)
        "#,
    )
    .bind(comment_id)
    .bind(comment_to_id)
    .execute(db())
    .await?;
    Ok(())
}

pub async fn list_comments_by_topic_id(
    topic_id: i32,
    current_user_id: i32,
) -> Result<Vec<TopicWithStats>, sqlx::Error> {
    let comments: Vec<TopicWithStats> = sqlx::query_as(
        r#"
        SELECT
            t.id,
            t.user_id,
            t.title,
            t.content,
            t.images,
            t.create_at,
            u.username AS user_name,
            u.email AS user_email,
            u.profile AS user_profile,
            COALESCE(c.comment_count, 0)::BIGINT AS comment_count,
            COALESCE(l.like_count, 0)::BIGINT AS like_count,
            EXISTS(
                SELECT 1 FROM topic_like tl2
                WHERE tl2.topic_id = t.id AND tl2.user_id = $2
            ) AS liked
        FROM reply r
        JOIN topic t ON t.id = r.comment_id
        JOIN _user u ON u.id = t.user_id
        LEFT JOIN (
            SELECT comment_to_id, COUNT(*)::BIGINT AS comment_count
            FROM reply
            GROUP BY comment_to_id
        ) c ON c.comment_to_id = t.id
        LEFT JOIN (
            SELECT topic_id, COUNT(*)::BIGINT AS like_count
            FROM topic_like
            GROUP BY topic_id
        ) l ON l.topic_id = t.id
        WHERE r.comment_to_id = $1
        ORDER BY t.create_at ASC, t.id ASC
        "#,
    )
    .bind(topic_id)
    .bind(current_user_id)
    .fetch_all(db())
    .await?;
    Ok(comments)
}
