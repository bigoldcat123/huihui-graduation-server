use faithea::{data::Json, get, post};
use faithea::data::inbound::FromRequest;

use crate::{
    model::{ApiResponse, input::SetCalorieGoalInput},
    service::{self, auth::CurrentUserId},
};

#[get("/")]
async fn get(user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> =
        service::user_calorie_goal::get_current_calorie_goal(user_id.into_inner().0).await.into();
    res.json()
}

#[post("/")]
async fn set(user_id: FromRequest<CurrentUserId>, ipt: Json<SetCalorieGoalInput>) {
    let res: ApiResponse<_> =
        service::user_calorie_goal::set_calorie_goal(user_id.into_inner().0, ipt.0).await.into();
    res.json()
}