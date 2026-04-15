use faithea::{data::{Json, inbound::FromRequest}, get, post};

use crate::{model::{ApiResponse, input::{CreateTopicInput, TopicLikeInput}}, service::{self, auth::CurrentUserId}};

#[get("/")]
async fn list_topics(#[search_param] page: Option<i64>, user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::topic::list(page, user_id.into_inner().0).await.into();
    res.json()
}

#[post("/")]
async fn create_topic(ipt: Json<CreateTopicInput>, user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::topic::create(user_id.into_inner().0, ipt.0).await.into();
    res.json()
}

#[post("/like")]
async fn like_topic(ipt: Json<TopicLikeInput>, user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::topic::set_like(user_id.into_inner().0, ipt.0).await.into();
    res.json()
}

#[get("/comment/{topic_id}")]
async fn list_comment(topic_id: i32, user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::topic::list_comment(topic_id, user_id.into_inner().0).await.into();
    res.json()
}

#[get("/my")]
async fn list_my_topics(user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::topic::list_my_topics(user_id.into_inner().0).await.into();
    res.json()
}

#[post("/delete/{topic_id}")]
async fn delete_topic(topic_id: i32, user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::topic::delete(topic_id, user_id.into_inner().0).await.into();
    res.json()
}
