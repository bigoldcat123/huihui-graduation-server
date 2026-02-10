use crate::{db, model::raw::Tag};



pub async fn list_tags() -> Result<Vec<Tag>, sqlx::Error> {
    let tags: Vec<Tag> = sqlx::query_as("SELECT * FROM tag ORDER BY id")
        .fetch_all(db())
        .await?;
    Ok(tags)
}
