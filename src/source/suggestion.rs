use crate::db;
use crate::model::raw::Suggestion;

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

pub async fn get_suggestion_by_id(suggestion_id: i32) -> Result<Suggestion, sqlx::Error> {
    let suggestion: Suggestion = sqlx::query_as(
        r#"
        SELECT
            id,
            content,
            images,
            type::text AS type,
            status::text AS status,
            food_id,
            restaurant_id,
            reviewer_id,
            review_comment,
            user_id,
            created_at,
            reviewed_at
        FROM suggestion
        WHERE id = $1
        "#,
    )
    .bind(suggestion_id)
    .fetch_one(db())
    .await?;
    Ok(suggestion)
}

pub async fn list_my_suggestions(user_id: i32) -> Result<Vec<Suggestion>, sqlx::Error> {
    let suggestions: Vec<Suggestion> = sqlx::query_as(
        r#"
        SELECT
            id,
            content,
            images,
            type::text AS type,
            status::text AS status,
            food_id,
            restaurant_id,
            reviewer_id,
            review_comment,
            user_id,
            created_at,
            reviewed_at
        FROM suggestion
        WHERE user_id = $1
        ORDER BY created_at DESC, id DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(db())
    .await?;
    Ok(suggestions)
}

pub async fn list_suggestions_by_page(
    page: i64,
    page_size: i64,
    status: Option<&str>,
    suggestion_type: Option<&str>,
) -> Result<Vec<Suggestion>, sqlx::Error> {
    let page = if page < 1 { 1 } else { page };
    let page_size = page_size.clamp(1, 100);
    let offset = (page - 1) * page_size;
    let suggestions: Vec<Suggestion> = sqlx::query_as(
        r#"
        SELECT
            id,
            content,
            images,
            type::text AS type,
            status::text AS status,
            food_id,
            restaurant_id,
            reviewer_id,
            review_comment,
            user_id,
            created_at,
            reviewed_at
        FROM suggestion
        WHERE ($3::suggestion_status IS NULL OR status = $3::suggestion_status)
          AND ($4::suggestion_type IS NULL OR type = $4::suggestion_type)
        ORDER BY created_at DESC, id DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size)
    .bind(offset)
    .bind(status)
    .bind(suggestion_type)
    .fetch_all(db())
    .await?;
    Ok(suggestions)
}

pub async fn review_suggestion(
    suggestion_id: i32,
    reviewer_id: i32,
    status: &str,
    review_comment: &str,
) -> Result<(), sqlx::Error> {
    let affected = sqlx::query(
        r#"
        UPDATE suggestion
        SET
            status = $1::suggestion_status,
            reviewer_id = $2,
            review_comment = $3,
            reviewed_at = CURRENT_TIMESTAMP
        WHERE id = $4
          AND status = 'PENDING'
        "#,
    )
    .bind(status)
    .bind(reviewer_id)
    .bind(review_comment)
    .bind(suggestion_id)
    .execute(db())
    .await?
    .rows_affected();

    if affected == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}
