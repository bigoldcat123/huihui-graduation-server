use std::collections::HashSet;

use crate::{db, model::raw::Tag};
use serde::Serialize;
use sqlx::FromRow;


#[derive(Debug, Clone, FromRow, Serialize)]
pub struct FoodRow {
    pub id: i32,
    pub restaurant_id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
}

pub async fn init_suggest(tags: Vec<Tag>) -> Result<Vec<FoodRow>, sqlx::Error> {
    let mut picked: HashSet<i32> = HashSet::new();
    let mut result: Vec<FoodRow> = Vec::with_capacity(tags.len());

    for tag in tags.into_iter() {
        let picked_ids: Vec<i32> = picked.iter().copied().collect();
        let row: Option<FoodRow> = sqlx::query_as(
            r#"
            SELECT f.id, f.restaurant_id, f.name, f.description, f.image
            FROM food f
            JOIN food_tag ft ON ft.food_id = f.id
            WHERE ft.tag_id = $1
              AND ($2::int[] IS NULL OR f.id != ALL($2))
            ORDER BY f.id
            LIMIT 1
            "#,
        )
        .bind(tag.id)
        .bind(if picked_ids.is_empty() { None } else { Some(picked_ids) })
        .fetch_optional(db())
        .await?;

        let row = row.ok_or(sqlx::Error::RowNotFound)?;
        picked.insert(row.id);
        result.push(row);
    }

    Ok(result)
}

pub async fn list_foods() -> Result<Vec<FoodRow>, sqlx::Error> {
    let foods: Vec<FoodRow> = sqlx::query_as(
        r#"
        SELECT id, restaurant_id, name, description, image
        FROM food
        ORDER BY id
        "#,
    )
    .fetch_all(db())
    .await?;
    Ok(foods)
}

pub async fn list_foods_by_page(page: i64, page_size: i64) -> Result<Vec<FoodRow>, sqlx::Error> {
    let page = if page < 1 { 1 } else { page };
    let offset = (page - 1) * page_size;
    let foods: Vec<FoodRow> = sqlx::query_as(
        r#"
        SELECT id, restaurant_id, name, description, image
        FROM food
        ORDER BY id
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(db())
    .await?;
    Ok(foods)
}

pub async fn list_food_not_in_ids(ids: &Vec<i32>) -> Result<Vec<FoodRow>, sqlx::Error> {
    if ids.is_empty() {
        return list_foods().await;
    }

    let foods: Vec<FoodRow> = sqlx::query_as(
        r#"
        SELECT id, restaurant_id, name, description, image
        FROM food
        WHERE id != ALL($1)
        ORDER BY id
        "#,
    )
    .bind(ids)
    .fetch_all(db())
    .await?;
    Ok(foods)
}

pub async fn list_food_in_ids(ids: &Vec<i32>) -> Result<Vec<FoodRow>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let foods: Vec<FoodRow> = sqlx::query_as(
        r#"
        SELECT id, restaurant_id, name, description, image
        FROM food
        WHERE id = ANY($1)
        ORDER BY id
        "#,
    )
    .bind(ids)
    .fetch_all(db())
    .await?;
    Ok(foods)
}

pub async fn list_user_liked_foods(_user_id: i32) -> Result<Vec<FoodRow>, sqlx::Error> {
    let foods: Vec<FoodRow> = sqlx::query_as(
        r#"
        SELECT
            f.id, f.restaurant_id, f.name, f.description, f.image
        FROM operation o
        JOIN food f ON f.id = o.food_id
        WHERE o.user_id = $1
          AND o.name = 'like'
          AND o.weight > 0
        ORDER BY o.id DESC
        "#,
    )
    .bind(_user_id)
    .fetch_all(db())
    .await?;
    Ok(foods)
}

pub async fn create_food(
    restaurant_id: i32,
    name: &str,
    description: &str,
    image: &str,
) -> Result<FoodRow, sqlx::Error> {
    let food: FoodRow = sqlx::query_as(
        r#"
        INSERT INTO food (restaurant_id, name, description, image)
        VALUES ($1, $2, $3, $4)
        RETURNING id, restaurant_id, name, description, image
        "#,
    )
    .bind(restaurant_id)
    .bind(name)
    .bind(description)
    .bind(image)
    .fetch_one(db())
    .await?;
    Ok(food)
}

pub async fn add_food_tag(food_id: i32, tag_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO food_tag (food_id, tag_id)
        VALUES ($1, $2)
        ON CONFLICT (food_id, tag_id) DO NOTHING
        "#,
    )
    .bind(food_id)
    .bind(tag_id)
    .execute(db())
    .await?;
    Ok(())
}

pub async fn clear_food_tags(food_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM food_tag
        WHERE food_id = $1
        "#,
    )
    .bind(food_id)
    .execute(db())
    .await?;
    Ok(())
}

pub async fn update_food(
    id: i32,
    restaurant_id: i32,
    name: &str,
    description: &str,
    image: &str,
) -> Result<FoodRow, sqlx::Error> {
    let food: FoodRow = sqlx::query_as(
        r#"
        UPDATE food
        SET
            restaurant_id = $2,
            name = $3,
            description = $4,
            image = $5
        WHERE id = $1
        RETURNING id, restaurant_id, name, description, image
        "#,
    )
    .bind(id)
    .bind(restaurant_id)
    .bind(name)
    .bind(description)
    .bind(image)
    .fetch_one(db())
    .await?;
    Ok(food)
}
