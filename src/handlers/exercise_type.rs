use faithea::{data::Json, get, post};
use crate::{
    model::{ApiResponse, input::{CreateExerciseTypeInput, UpdateExerciseTypeInput}},
    service,
};

#[get("/")]
async fn list() {
    let res: ApiResponse<_> = service::exercise_type::list().await.into();
    res.json()
}

#[get("/{id}")]
async fn get_by_id(id: i32) {
    let res: ApiResponse<_> = service::exercise_type::get_by_id(id).await.into();
    res.json()
}

#[post("/")]
async fn create(ipt: Json<CreateExerciseTypeInput>) {
    let res: ApiResponse<_> = service::exercise_type::create(ipt.0).await.into();
    res.json()
}

#[post("/update")]
async fn update(ipt: Json<UpdateExerciseTypeInput>) {
    let res: ApiResponse<_> = service::exercise_type::update(ipt.0).await.into();
    res.json()
}
#[post("/delete/{id}")]
async fn delete(id: i32) {
    let res: ApiResponse<_> = service::exercise_type::delete(id).await.into();
    res.json()
}
