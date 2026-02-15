use faithea::{data::Json, get, post};

use crate::{model::{ApiResponse, input::CreateTagInput}, service};

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
