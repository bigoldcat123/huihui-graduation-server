use faithea::{get};

use crate::{model::ApiResponse, service};

#[get("/initSuggest")]
async fn init_suggest() {
    let res: ApiResponse<_> = service::food::init_suggest().await.into();
    res.json()
}
