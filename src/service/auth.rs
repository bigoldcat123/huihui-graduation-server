use crate::{
    model::{input::{RegisterInput, UsernamePasswordAuthentication}, output::CurrentUser},
    service::error::ServiceError,
    source,
};

pub async fn login(auth: UsernamePasswordAuthentication) -> Result<CurrentUser, ServiceError> {
    let user = source::user::get_user_by_name(&auth.username).await?;

    Ok(user.into())
}

pub async fn register(input: RegisterInput) -> Result<CurrentUser, ServiceError> {
    let user = source::user::create_user(&input.email, &input.username, &input.password).await?;

    Ok(user.into())
}
