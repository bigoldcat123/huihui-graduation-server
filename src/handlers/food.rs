use faithea::{data::Json, get, post};

use crate::{model::ApiResponse, service};

#[get("/initSuggest")]
async fn init_suggest() {
    let res: ApiResponse<_> = service::food::init_suggest().await.into();
    res.json()
}

#[post("/consecutiveSuggest")]
async fn consecutive_suggest(food_ids: Json<Vec<i32>>) {
    let res: ApiResponse<_> = service::food::consecutive_suggest(food_ids.0).await.into();
    res.json()
}
