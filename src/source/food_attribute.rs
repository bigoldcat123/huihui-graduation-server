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

#[cfg(test)]
mod tests {
    use crate::init_db_if_not;

    use super::*;
    use crate::source::{food, restaurant};

    async fn setup_test_food() -> i32 {
        println!("[test] About to create restaurant...");
        let rest = restaurant::create_restaurant(
            "Test Restaurant",
            None,
            "Test Location",
            "test.jpg",
        )
        .await
        .expect("create_restaurant failed");
        println!("[test] Restaurant created with id: {}", rest.id);
        let f = food::create_food(rest.id, "Test Food", "desc", "img.jpg", 10.0)
            .await
            .expect("create_food failed");
        println!("[test] Food created with id: {}", f.id);
        f.id
    }

    fn test_attr(food_id: i32) -> FoodAttributeRow {
        FoodAttributeRow {
            food_id,
            calories: 500.0,
            protein: 30.0,
            fat: 20.0,
            carbohydrates: 50.0,
            fiber: 5.0,
            sugar: 10.0,
            sodium: 800.0,
            serving_size: "200g".to_string(),
            is_vegetarian: false,
            is_vegan: false,
            is_gluten_free: true,
            allergens: "nuts".to_string(),
            ingredients: "chicken, rice, vegetables".to_string(),
        }
    }

    #[tokio::test]
    async fn test_upsert_and_get() {
        init_db_if_not().await;
        let food_id = setup_test_food().await;
        let attr = test_attr(food_id);
        upsert_food_attribute(&attr).await.unwrap();

        let fetched = get_food_attribute(food_id).await.unwrap();
        assert_eq!(fetched.food_id, food_id);
        assert_eq!(fetched.calories, 500.0);
        assert_eq!(fetched.protein, 30.0);
        assert_eq!(fetched.serving_size, "200g");
        assert!(!fetched.is_vegetarian);
        assert!(fetched.is_gluten_free);
        assert_eq!(fetched.allergens, "nuts");
        assert_eq!(fetched.ingredients, "chicken, rice, vegetables");
    }

    #[tokio::test]
    async fn test_update() {
        init_db_if_not().await;
        let food_id = setup_test_food().await;
        let attr = test_attr(food_id);
        upsert_food_attribute(&attr).await.unwrap();

        let updated = FoodAttributeRow {
            food_id,
            calories: 600.0,
            protein: 35.0,
            fat: 25.0,
            carbohydrates: 55.0,
            fiber: 6.0,
            sugar: 12.0,
            sodium: 900.0,
            serving_size: "250g".to_string(),
            is_vegetarian: true,
            is_vegan: true,
            is_gluten_free: false,
            allergens: "dairy".to_string(),
            ingredients: "tofu, broccoli".to_string(),
        };
        update_food_attribute(food_id, &updated).await.unwrap();

        let fetched = get_food_attribute(food_id).await.unwrap();
        assert_eq!(fetched.calories, 600.0);
        assert_eq!(fetched.protein, 35.0);
        assert!(fetched.is_vegetarian);
        assert!(fetched.is_vegan);
        assert!(!fetched.is_gluten_free);
    }

    #[tokio::test]
    async fn test_delete() {
        init_db_if_not().await;
        let food_id = setup_test_food().await;
        let attr = test_attr(food_id);
        upsert_food_attribute(&attr).await.unwrap();

        delete_food_attribute(food_id).await.unwrap();

        let result = get_food_attribute(food_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_upsert_updates_existing() {
        init_db_if_not().await;
        let food_id = setup_test_food().await;
        let attr = test_attr(food_id);
        upsert_food_attribute(&attr).await.unwrap();

        let updated = FoodAttributeRow {
            food_id,
            calories: 999.0,
            protein: 99.0,
            fat: 99.0,
            carbohydrates: 99.0,
            fiber: 9.0,
            sugar: 9.0,
            sodium: 999.0,
            serving_size: "100g".to_string(),
            is_vegetarian: true,
            is_vegan: true,
            is_gluten_free: true,
            allergens: "".to_string(),
            ingredients: "updated".to_string(),
        };
        upsert_food_attribute(&updated).await.unwrap();

        let fetched = get_food_attribute(food_id).await.unwrap();
        assert_eq!(fetched.calories, 999.0);
        assert_eq!(fetched.ingredients, "updated");
    }
}
