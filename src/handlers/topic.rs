use faithea::{data::{Json, inbound::FromRequest}, get, post};

use crate::{model::{ApiResponse, input::CreateTopicInput}, service::{self, auth::CurrentUserId}};

#[get("/")]
async fn list_topics(#[search_param] page: Option<i64>) {
    let res: ApiResponse<_> = service::topic::list(page).await.into();
    res.json()
}

#[post("/")]
async fn create_topic(ipt: Json<CreateTopicInput>, user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::topic::create(user_id.into_inner().0, ipt.0).await.into();
    res.json()
}
