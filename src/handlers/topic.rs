use faithea::get;

use crate::{model::ApiResponse, service};

#[get("/")]
async fn list_topics(#[search_param] page: Option<String>) {
    let page = page.and_then(|v| v.parse::<i64>().ok());
    let res: ApiResponse<_> = service::topic::list(page).await.into();
    res.json()
}
