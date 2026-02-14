use faithea::get;

use crate::{model::ApiResponse, service};

#[get("/")]
async fn list_restaurants() {
    let res: ApiResponse<_> = service::restaurant::list().await.into();
    res.json()
}
