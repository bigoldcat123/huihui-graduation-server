use faithea::{data::Json, get, post};
use faithea::data::inbound::FromRequest;

use crate::{
    model::{ApiResponse, input::{MealRecordInput, InsertMealRecordFromFoodInput}},
    service::{self, auth::CurrentUserId},
};

#[get("/")]
async fn get_today(user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> =
        service::meal_record::get_today_records(user_id.into_inner().0).await.into();
    res.json()
}

#[post("/")]
async fn create(user_id: FromRequest<CurrentUserId>, ipt: Json<MealRecordInput>) {
    let res: ApiResponse<_> =
        service::meal_record::create_record(user_id.into_inner().0, ipt.0).await.into();
    res.json()
}

#[post("/inner")]
async fn insert_from_inner_food(user_id: FromRequest<CurrentUserId>, ipt: Json<InsertMealRecordFromFoodInput>) {
    let res: ApiResponse<_> =
        service::meal_record::insert_from_inner_food(user_id.into_inner().0, ipt.0).await.into();
    res.json()
}
