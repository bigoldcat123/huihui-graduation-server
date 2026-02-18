use crate::{
    model::output::{FoodSimple, Restaurant, RestaurantSimple},
    service::error::ServiceError,
    source,
};

pub async fn list() -> Result<Vec<Restaurant>, ServiceError> {
    let restaurants = source::restaurant::list_restaurants().await?;
    Ok(restaurants.into_iter().map(Restaurant::from).collect())
}

pub async fn list_simple() -> Result<Vec<RestaurantSimple>, ServiceError> {
    let restaurants = source::restaurant::list_restaurants().await?;
    Ok(restaurants.into_iter().map(RestaurantSimple::from).collect())
}

pub async fn list_foods_by_restaurant_id(
    restaurant_id: i32,
) -> Result<Vec<FoodSimple>, ServiceError> {
    let foods = source::food::list_foods_by_restaurant_id(restaurant_id).await?;
    Ok(foods
        .into_iter()
        .map(|food| FoodSimple {
            id: food.id,
            name: food.name,
            image: food.image,
        })
        .collect())
}
