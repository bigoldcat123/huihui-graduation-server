use crate::{
    model::input::CreateSuggestionInput,
    service::error::ServiceError,
    source,
};

pub async fn create(user_id: i32, ipt: CreateSuggestionInput) -> Result<i32, ServiceError> {
    let images_json = serde_json::to_string(&ipt.images)?;
    let new_id = source::suggestion::create_suggestion(
        user_id,
        &ipt.content,
        &images_json,
        ipt.r#type.as_db_str(),
        ipt.food_id,
        ipt.restaurant_id,
    )
    .await?;
    Ok(new_id)
}
