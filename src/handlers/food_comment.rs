use serde::Deserialize;
use faithea::{data::{Json, inbound::FromRequest}, get, post};

use crate::{model::ApiResponse, service::{self, auth::CurrentUserId}};

#[derive(Deserialize)]
pub struct CreateFoodCommentInput {
    pub content: String,
}

#[get("/{food_id}/comments")]
async fn get_food_comments(food_id: i32, token: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::food_comment::get_food_comments(food_id, token.0).await.into();
    res.json()
}

#[post("/{food_id}/comments")]
async fn create_food_comment(
    food_id: i32,
    ipt: Json<CreateFoodCommentInput>,
    token: FromRequest<CurrentUserId>,
) {
    let res: ApiResponse<_> = service::food_comment::create_food_comment(
        food_id,
        token.0,
        &ipt.content,
    )
    .await
    .into();
    res.json()
}

#[post("/comments/{comment_id}/thumb")]
async fn toggle_comment_thumb(comment_id: i32, token: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::food_comment::toggle_thumb(token.0, comment_id).await.into();
    res.json()
}
