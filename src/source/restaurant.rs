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

pub async fn list_restaurants_by_page(page: i64, page_size: i64) -> Result<Vec<Restaurant>, sqlx::Error> {
    let page = if page < 1 { 1 } else { page };
    let page_size = page_size.clamp(1, 100);
    let offset = (page - 1) * page_size;
    let restaurants: Vec<Restaurant> = sqlx::query_as(
        r#"
        SELECT id, name, description, location, image
        FROM restaurant
        ORDER BY id
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size)
    .bind(offset)
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

pub async fn create_restaurant(
    name: &str,
    description: Option<&str>,
    location: &str,
    image: &str,
) -> Result<Restaurant, sqlx::Error> {
    let restaurant: Restaurant = sqlx::query_as(
        r#"
        INSERT INTO restaurant (name, description, location, image)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, description, location, image
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(location)
    .bind(image)
    .fetch_one(db())
    .await?;
    Ok(restaurant)
}

pub async fn update_restaurant(
    id: i32,
    name: &str,
    description: Option<&str>,
    location: &str,
    image: &str,
) -> Result<Restaurant, sqlx::Error> {
    let restaurant: Restaurant = sqlx::query_as(
        r#"
        UPDATE restaurant
        SET name = $1, description = $2, location = $3, image = $4
        WHERE id = $5
        RETURNING id, name, description, location, image
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(location)
    .bind(image)
    .bind(id)
    .fetch_one(db())
    .await?;
    Ok(restaurant)
}
