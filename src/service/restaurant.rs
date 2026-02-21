use crate::{
    model::{
        input::CreateRestaurantInput,
        output::{FoodSimple, Restaurant, RestaurantSimple},
    },
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

pub async fn list_by_page(page: Option<i64>, page_size: Option<i64>) -> Result<Vec<Restaurant>, ServiceError> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);
    let restaurants = source::restaurant::list_restaurants_by_page(page, page_size).await?;
    Ok(restaurants.into_iter().map(Restaurant::from).collect())
}

pub async fn create(ipt: CreateRestaurantInput) -> Result<Restaurant, ServiceError> {
    let restaurant = source::restaurant::create_restaurant(
        &ipt.name,
        ipt.description.as_deref(),
        &ipt.location,
        &ipt.image,
    )
    .await?;
    Ok(Restaurant::from(restaurant))
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
