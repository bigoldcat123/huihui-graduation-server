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

pub async fn list_food_not_in_ids(ids: Vec<i32>) -> Result<Vec<FoodRow>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let foods: Vec<FoodRow> = sqlx::query_as(
        r#"
        SELECT id, restaurant_id, name, description, image
        FROM food
        WHERE id != ANY($1)
        ORDER BY id
        "#,
    )
    .bind(ids)
    .fetch_all(db())
    .await?;
    Ok(foods)
}
