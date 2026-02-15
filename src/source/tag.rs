use crate::{db, model::raw::Tag};



pub async fn list_tags() -> Result<Vec<Tag>, sqlx::Error> {
    let tags: Vec<Tag> = sqlx::query_as("SELECT * FROM tag ORDER BY id")
        .fetch_all(db())
        .await?;
    Ok(tags)
}

pub async fn list_food_tags(food_id: i32) -> Result<Vec<Tag>, sqlx::Error> {
    let tags: Vec<Tag> = sqlx::query_as(
        r#"
        SELECT t.id, t.name, t.image
        FROM tag t
        JOIN food_tag ft ON ft.tag_id = t.id
        WHERE ft.food_id = $1
        ORDER BY t.id
        "#,
    )
    .bind(food_id)
    .fetch_all(db())
    .await?;
    Ok(tags)
}

pub async fn create_tag(name: &str, image: &str) -> Result<Tag, sqlx::Error> {
    let tag: Tag = sqlx::query_as(
        r#"
        INSERT INTO tag (name, image)
        VALUES ($1, $2)
        RETURNING id, name, image
        "#,
    )
    .bind(name)
    .bind(image)
    .fetch_one(db())
    .await?;
    Ok(tag)
}
