use crate::{model::{input::CreateTagInput, output::NameValue, raw::Tag}, service::error::ServiceError, source};

pub async fn list() -> Result<Vec<Tag>, ServiceError> {
    let tags = source::tag::list_tags().await?;
    Ok(tags)
}

pub async fn create(ipt: CreateTagInput) -> Result<Tag, ServiceError> {
    let tag = source::tag::create_tag(&ipt.name, &ipt.image).await?;
    Ok(tag)
}

pub async fn list_user_liked_values(user_id: i32) -> Result<Vec<NameValue>, ServiceError> {
    let rows = source::tag::list_user_liked_tag_values(user_id).await?;
    Ok(rows
        .into_iter()
        .map(|r| NameValue {
            name: r.name,
            value: r.value,
        })
        .collect())
}
