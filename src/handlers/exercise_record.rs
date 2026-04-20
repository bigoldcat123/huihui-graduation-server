use faithea::{data::Json, get, post};
use faithea::data::inbound::FromRequest;

use crate::{
    model::{ApiResponse, input::CreateExerciseRecordInput},
    service::{self, auth::CurrentUserId},
};

#[get("/")]
async fn get_today(user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> =
        service::exercise_record::get_today_records(user_id.into_inner().0).await.into();
    res.json()
}

#[get("/all")]
async fn get_all(user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> =
        service::exercise_record::get_all_records(user_id.into_inner().0).await.into();
    res.json()
}

#[post("/")]
async fn create(user_id: FromRequest<CurrentUserId>, ipt: Json<CreateExerciseRecordInput>) {
    let res: ApiResponse<_> =
        service::exercise_record::create_record(user_id.into_inner().0, ipt.0).await.into();
    res.json()
}
