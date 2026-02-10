use faithea::{data::Json, post};

use crate::{model::{ApiResponse, input::UsernamePasswordAuthentication}, service};



#[post("/login")]
async fn login(auth:Json<UsernamePasswordAuthentication>) {
    let res:ApiResponse<_> = service::auth::login(auth.0).await.into();
    res.json()
}
