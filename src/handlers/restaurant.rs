use faithea::get;

use crate::{model::ApiResponse, service};

#[get("/")]
async fn list_restaurants() {
    let res: ApiResponse<_> = service::restaurant::list().await.into();
    res.json()
}

#[get("/simple")]
async fn list_restaurants_simple() {
    let res: ApiResponse<_> = service::restaurant::list_simple().await.into();
    res.json()
}

#[get("/foods")]
async fn list_foods_by_restaurant_id(#[search_param] restaurant_id: i32) {
    let res: ApiResponse<_> = service::restaurant::list_foods_by_restaurant_id(restaurant_id)
        .await
        .into();
    res.json()
}
