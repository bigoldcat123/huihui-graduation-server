use crate::{
    model::{input::UsernamePasswordAuthentication, output::CurrentUser},
    service::error::ServiceError,
    source,
};

pub async fn login(auth: UsernamePasswordAuthentication) -> Result<CurrentUser, ServiceError> {
    let user = source::user::get_user_by_name(&auth.username).await?;

    Ok(user.into())
}
