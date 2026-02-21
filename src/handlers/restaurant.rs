use faithea::{
    data::{Json, inbound::FromRequest},
    get, post,
};

use crate::{
    model::{ApiResponse, input::CreateRestaurantInput},
    service::{self, auth::CurrentRootUserId},
};

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

#[get("/list")]
async fn list_restaurants_by_page(
    #[search_param] page: Option<i64>,
    #[search_param] page_size: Option<i64>,
    _root: FromRequest<CurrentRootUserId>,
) {
    let res: ApiResponse<_> = service::restaurant::list_by_page(page, page_size).await.into();
    res.json()
}

#[post("/")]
async fn create_restaurant(ipt: Json<CreateRestaurantInput>, _root: FromRequest<CurrentRootUserId>) {
    let res: ApiResponse<_> = service::restaurant::create(ipt.0).await.into();
    res.json()
}
