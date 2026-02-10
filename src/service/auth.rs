use crate::{
    model::{
        input::{RegisterInput, UsernamePasswordAuthentication},
        output::{AuthResult, CurrentUser},
    },
    service::error::ServiceError,
    source,
};
use chrono::{Duration, Utc};
use faithea::{handler::types::HttpHandlerError, header::AUTHORIZATION, request::TryFromRequest};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: usize,
}

pub struct CurrentUserId(pub i32);

impl<'a> TryFromRequest<'a> for CurrentUserId {
    fn try_from_request(
        req: &'a mut faithea::request::HttpRequest,
    ) -> Result<Self, faithea::handler::types::HttpHandlerError> {
        let token = req
            .get_header(AUTHORIZATION)
            .ok_or(faithea::handler::types::HttpHandlerError::before_handler_param_not_exist())?;
        let token = token
            .to_str()
            .map_err(|_| {
                faithea::handler::types::HttpHandlerError::before_handler_param_not_exist()
            })?
            .split_once(" ")
            .ok_or(HttpHandlerError::before_handler_param_not_exist())?
            .1;
        let claims =
            verify_token(token).map_err(|_| HttpHandlerError::before_handler_param_not_exist())?;
        Ok(CurrentUserId(claims.sub))
    }
}

fn jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me".to_string())
}

fn sign_token(user_id: i32) -> Result<String, ServiceError> {
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims { sub: user_id, exp };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )?;
    Ok(token)
}

fn verify_token(token: &str) -> Result<Claims, ServiceError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

pub async fn login(auth: UsernamePasswordAuthentication) -> Result<AuthResult, ServiceError> {
    let user = source::user::get_user_by_name(&auth.username).await?;

    let current_user: CurrentUser = user.into();
    let token = sign_token(current_user.id)?;
    Ok(AuthResult { token })
}

pub async fn register(input: RegisterInput) -> Result<AuthResult, ServiceError> {
    let user = source::user::create_user(&input.email, &input.username, &input.password).await?;

    let current_user: CurrentUser = user.into();
    let token = sign_token(current_user.id)?;
    Ok(AuthResult { token })
}

pub async fn me(id: i32) -> Result<CurrentUser, ServiceError> {
    let user = source::user::get_user_by_id(id).await?;
    Ok(user.into())
}
