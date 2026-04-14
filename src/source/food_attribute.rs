use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::db;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FoodAttributeRow {
    pub food_id: i32,
    pub calories: f64,
    pub protein: f64,
    pub fat: f64,
    pub carbohydrates: f64,
    pub fiber: f64,
    pub sugar: f64,
    pub sodium: f64,
    pub serving_size: String,
    pub is_vegetarian: bool,
    pub is_vegan: bool,
    pub is_gluten_free: bool,
    pub allergens: String,
    pub ingredients: String,
}

pub async fn get_food_attribute(food_id: i32) -> Result<FoodAttributeRow, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT
            food_id,
            calories::float8, protein::float8, fat::float8,
            carbohydrates::float8, fiber::float8, sugar::float8, sodium::float8,
            serving_size, is_vegetarian, is_vegan, is_gluten_free,
            allergens, ingredients
        FROM food_attribute
        WHERE food_id = $1
        "#,
    )
    .bind(food_id)
    .fetch_one(db())
    .await
}

pub async fn create_food_attribute(attr: &FoodAttributeRow) -> Result<FoodAttributeRow, sqlx::Error> {
    sqlx::query_as(
        r#"
        INSERT INTO food_attribute (
            food_id, calories, protein, fat, carbohydrates,
            fiber, sugar, sodium, serving_size,
            is_vegetarian, is_vegan, is_gluten_free,
            allergens, ingredients
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        RETURNING
            food_id,
            calories::float8, protein::float8, fat::float8,
            carbohydrates::float8, fiber::float8, sugar::float8, sodium::float8,
            serving_size, is_vegetarian, is_vegan, is_gluten_free,
            allergens, ingredients
        "#,
    )
    .bind(attr.food_id)
    .bind(attr.calories)
    .bind(attr.protein)
    .bind(attr.fat)
    .bind(attr.carbohydrates)
    .bind(attr.fiber)
    .bind(attr.sugar)
    .bind(attr.sodium)
    .bind(&attr.serving_size)
    .bind(attr.is_vegetarian)
    .bind(attr.is_vegan)
    .bind(attr.is_gluten_free)
    .bind(&attr.allergens)
    .bind(&attr.ingredients)
    .fetch_one(db())
    .await
}

pub async fn update_food_attribute(
    food_id: i32,
    attr: &FoodAttributeRow,
) -> Result<FoodAttributeRow, sqlx::Error> {
    sqlx::query_as(
        r#"
        UPDATE food_attribute
        SET
            calories = $2,
            protein = $3,
            fat = $4,
            carbohydrates = $5,
            fiber = $6,
            sugar = $7,
            sodium = $8,
            serving_size = $9,
            is_vegetarian = $10,
            is_vegan = $11,
            is_gluten_free = $12,
            allergens = $13,
            ingredients = $14
        WHERE food_id = $1
        RETURNING
            food_id,
            calories::float8, protein::float8, fat::float8,
            carbohydrates::float8, fiber::float8, sugar::float8, sodium::float8,
            serving_size, is_vegetarian, is_vegan, is_gluten_free,
            allergens, ingredients
        "#,
    )
    .bind(food_id)
    .bind(attr.calories)
    .bind(attr.protein)
    .bind(attr.fat)
    .bind(attr.carbohydrates)
    .bind(attr.fiber)
    .bind(attr.sugar)
    .bind(attr.sodium)
    .bind(&attr.serving_size)
    .bind(attr.is_vegetarian)
    .bind(attr.is_vegan)
    .bind(attr.is_gluten_free)
    .bind(&attr.allergens)
    .bind(&attr.ingredients)
    .fetch_one(db())
    .await
}

pub async fn upsert_food_attribute(attr: &FoodAttributeRow) -> Result<FoodAttributeRow, sqlx::Error> {
    sqlx::query_as(
        r#"
        INSERT INTO food_attribute (
            food_id, calories, protein, fat, carbohydrates,
            fiber, sugar, sodium, serving_size,
            is_vegetarian, is_vegan, is_gluten_free,
            allergens, ingredients
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        ON CONFLICT (food_id) DO UPDATE SET
            calories = EXCLUDED.calories,
            protein = EXCLUDED.protein,
            fat = EXCLUDED.fat,
            carbohydrates = EXCLUDED.carbohydrates,
            fiber = EXCLUDED.fiber,
            sugar = EXCLUDED.sugar,
            sodium = EXCLUDED.sodium,
            serving_size = EXCLUDED.serving_size,
            is_vegetarian = EXCLUDED.is_vegetarian,
            is_vegan = EXCLUDED.is_vegan,
            is_gluten_free = EXCLUDED.is_gluten_free,
            allergens = EXCLUDED.allergens,
            ingredients = EXCLUDED.ingredients
        RETURNING
            food_id,
            calories::float8, protein::float8, fat::float8,
            carbohydrates::float8, fiber::float8, sugar::float8, sodium::float8,
            serving_size, is_vegetarian, is_vegan, is_gluten_free,
            allergens, ingredients
        "#,
    )
    .bind(attr.food_id)
    .bind(attr.calories)
    .bind(attr.protein)
    .bind(attr.fat)
    .bind(attr.carbohydrates)
    .bind(attr.fiber)
    .bind(attr.sugar)
    .bind(attr.sodium)
    .bind(&attr.serving_size)
    .bind(attr.is_vegetarian)
    .bind(attr.is_vegan)
    .bind(attr.is_gluten_free)
    .bind(&attr.allergens)
    .bind(&attr.ingredients)
    .fetch_one(db())
    .await
}

pub async fn delete_food_attribute(food_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query(r#"DELETE FROM food_attribute WHERE food_id = $1"#)
        .bind(food_id)
        .execute(db())
        .await?;
    Ok(())
}
