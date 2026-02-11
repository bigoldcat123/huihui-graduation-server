use faithea::{data::Json, post};

use crate::{model::{ApiResponse, input::SuggestionInput}, service};

#[post("/consecutiveSuggest")]
async fn consecutive_suggest(food_ids: Json<SuggestionInput>) {
    let res: ApiResponse<_> = service::food::consecutive_suggest(food_ids.0).await.into();
    res.json()
}
