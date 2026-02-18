use faithea::{data::{Json, inbound::FromRequest}, get, post};

use crate::{model::{ApiResponse, input::CreateSuggestionInput}, service::{self, auth::CurrentUserId}};

#[post("/")]
async fn create_suggestion(ipt: Json<CreateSuggestionInput>, user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::suggestion::create(user_id.into_inner().0, ipt.0).await.into();
    res.json()
}

#[get("/my")]
async fn list_my_suggestion(user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::suggestion::list_my(user_id.into_inner().0).await.into();
    res.json()
}
