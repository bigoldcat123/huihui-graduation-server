use crate::{model::output::Restaurant, service::error::ServiceError, source};

pub async fn list() -> Result<Vec<Restaurant>, ServiceError> {
    let restaurants = source::restaurant::list_restaurants().await?;
    Ok(restaurants.into_iter().map(Restaurant::from).collect())
}
