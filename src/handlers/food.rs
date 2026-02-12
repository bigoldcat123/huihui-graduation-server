use faithea::{data::{Json, inbound::FromRequest}, post};

use crate::{model::{ApiResponse, input::SuggestionInput}, service::{self, auth::CurrentUserId}};

#[post("/consecutiveSuggest")]
async fn consecutive_suggest(food_ids: Json<SuggestionInput>, token: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::food::consecutive_suggest(food_ids.0,token.into_inner().0).await.into();
    res.json()
}
