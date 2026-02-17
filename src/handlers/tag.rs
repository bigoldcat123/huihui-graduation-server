use faithea::{data::Json, get, post};

use crate::{model::{ApiResponse, input::CreateTagInput}, service::{self, auth::CurrentUserId}};
use faithea::data::inbound::FromRequest;

#[get("/")]
async fn list_tags() {
    let res: ApiResponse<_> = service::tag::list().await.into();
    res.json()
}

#[post("/")]
async fn create_tag(ipt: Json<CreateTagInput>) {
    let res: ApiResponse<_> = service::tag::create(ipt.0).await.into();
    res.json()
}

#[get("/liked-values")]
async fn list_liked_values(user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::tag::list_user_liked_values(user_id.into_inner().0).await.into();
    res.json()
}
