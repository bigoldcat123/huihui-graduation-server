use faithea::get;

use crate::{model::ApiResponse, service};

#[get("/")]
async fn list_tags() {
    let res: ApiResponse<_> = service::tag::list().await.into();
    res.json()
}
