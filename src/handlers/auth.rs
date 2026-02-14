use faithea::{data::{Json, inbound::FromRequest}, get, post};

use crate::{model::{ApiResponse, input::{RegisterInput, UpdateUserInfoInput, UsernamePasswordAuthentication}}, service::{self, auth::CurrentUserId}};



#[post("/login")]
async fn login(auth:Json<UsernamePasswordAuthentication>) {
    let res:ApiResponse<_> = service::auth::login(auth.0).await.into();
    res.json()
}

#[post("/login/root")]
async fn root_login(auth: Json<UsernamePasswordAuthentication>) {
    let res: ApiResponse<_> = service::auth::root_login(auth.0).await.into();
    res.json()
}

#[post("/register")]
async fn register(input: Json<RegisterInput>) {
    let res: ApiResponse<_> = service::auth::register(input.0).await.into();
    res.json()
}

#[get("/me")]
async fn me(user_id:FromRequest<CurrentUserId>,_token:FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::auth::me(user_id.into_inner().0).await.into();
    res.json()
}

#[post("/update")]
async fn update_user_info(ipt: Json<UpdateUserInfoInput>, user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::auth::update_user_info(user_id.into_inner().0, ipt.0).await.into();
    res.json()
}
