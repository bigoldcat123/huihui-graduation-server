use crate::{db, model::raw::TodoLog};

pub async fn list_todo_logs_by_suggestion_and_status(
    suggestion_id: i32,
    suggestion_status: &str,
) -> Result<Vec<TodoLog>, sqlx::Error> {
    let rows: Vec<TodoLog> = sqlx::query_as(
        r#"
        SELECT
            id,
            suggestion_id,
            suggestion_status::text AS suggestion_status,
            content,
            create_time
        FROM todo_log
        WHERE suggestion_id = $1
          AND suggestion_status = $2::suggestion_status
        ORDER BY id DESC
        "#,
    )
    .bind(suggestion_id)
    .bind(suggestion_status)
    .fetch_all(db())
    .await?;
    Ok(rows)
}

pub async fn create_todo_log(
    suggestion_id: i32,
    suggestion_status: &str,
    content: &str,
) -> Result<i32, sqlx::Error> {
    let new_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO todo_log (suggestion_id, suggestion_status, content)
        VALUES ($1, $2::suggestion_status, $3)
        RETURNING id
        "#,
    )
    .bind(suggestion_id)
    .bind(suggestion_status)
    .bind(content)
    .fetch_one(db())
    .await?;
    Ok(new_id)
}
