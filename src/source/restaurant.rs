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

pub async fn get_restaurant_by_id(id: i32) -> Result<Restaurant, sqlx::Error> {
    let restaurant: Restaurant = sqlx::query_as(
        r#"
        SELECT id, name, description, location, image
        FROM restaurant
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(db())
    .await?;
    Ok(restaurant)
}
