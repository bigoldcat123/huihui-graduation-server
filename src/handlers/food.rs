use faithea::{data::Json, get, post};

use crate::{model::{ApiResponse, input::SuggestionInput}, service};

// #[get("/initSuggest")]
// async fn init_suggest() {
//     let res: ApiResponse<_> = service::food::init_suggest().await.into();
//     res.json()
// }

#[post("/initSuggest")]
async fn init_suggest(food_ids: Json<SuggestionInput>) {
    let res: ApiResponse<_> = service::food::consecutive_suggest(food_ids.0).await.into();
    res.json()
}
