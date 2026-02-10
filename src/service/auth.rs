use crate::{
    model::{
        input::{RegisterInput, UsernamePasswordAuthentication},
        output::{AuthResult, CurrentUser},
    },
    service::error::ServiceError,
    source,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: usize,
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

pub async fn login(auth: UsernamePasswordAuthentication) -> Result<AuthResult, ServiceError> {
    let user = source::user::get_user_by_name(&auth.username).await?;

    let current_user: CurrentUser = user.into();
    let token = sign_token(current_user.id)?;
    Ok(AuthResult {
        token,
    })
}

pub async fn register(input: RegisterInput) -> Result<AuthResult, ServiceError> {
    let user = source::user::create_user(&input.email, &input.username, &input.password).await?;

    let current_user: CurrentUser = user.into();
    let token = sign_token(current_user.id)?;
    Ok(AuthResult {
        token,
    })
}
