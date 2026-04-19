use crate::{model::input::UserProfileInput, model::output::UserProfileOutput, service::error::ServiceError, source};
use chrono::NaiveDate;

pub async fn get_user_profile(user_id: i32) -> Result<Option<UserProfileOutput>, ServiceError> {
    let profile = source::user_profile::get_user_profile(user_id).await?;
    Ok(profile.map(|p| p.into()))
}

pub async fn update_user_profile(
    user_id: i32,
    input: UserProfileInput,
) -> Result<UserProfileOutput, ServiceError> {
    log::info!("{input:?}");
    let birth_date = NaiveDate::parse_from_str(&input.birth_date, "%Y-%m-%d").ok();
    let profile = source::user_profile::upsert_user_profile(
        user_id,
        input.height_cm,
        input.weight_kg,
        birth_date,
        &input.gender,
    )
    .await?;
    Ok(profile.into())
}
