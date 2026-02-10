use faithea::{data::Json, post};

use crate::{model::{ApiResponse, input::{RegisterInput, UsernamePasswordAuthentication}}, service};



#[post("/login")]
async fn login(auth:Json<UsernamePasswordAuthentication>) {
    let res:ApiResponse<_> = service::auth::login(auth.0).await.into();
    res.json()
}

#[post("/register")]
async fn register(input: Json<RegisterInput>) {
    let res: ApiResponse<_> = service::auth::register(input.0).await.into();
    res.json()
}
