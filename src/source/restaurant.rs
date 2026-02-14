use crate::{db, model::raw::Restaurant};

pub async fn list_restaurants() -> Result<Vec<Restaurant>, sqlx::Error> {
    let restaurants: Vec<Restaurant> = sqlx::query_as(
        r#"
        SELECT id, name, description, location, image
        FROM restaurant
        ORDER BY id
        "#,
    )
    .fetch_all(db())
    .await?;
    Ok(restaurants)
}
