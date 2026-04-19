use faithea::{data::Json, get, post};
use faithea::data::inbound::FromRequest;

use crate::{
    model::{ApiResponse, input::UserProfileInput},
    service::{self, auth::CurrentUserId},
};

#[get("/")]
async fn get(user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::user_profile::get_user_profile(user_id.into_inner().0).await.into();
    res.json()
}

#[post("/")]
async fn update(user_id: FromRequest<CurrentUserId>, ipt: Json<UserProfileInput>) {
    let res: ApiResponse<_> = service::user_profile::update_user_profile(user_id.into_inner().0, ipt.0).await.into();
    res.json()
}
