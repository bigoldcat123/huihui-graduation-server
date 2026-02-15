use crate::{model::{input::CreateTagInput, raw::Tag}, service::error::ServiceError, source};

pub async fn list() -> Result<Vec<Tag>, ServiceError> {
    let tags = source::tag::list_tags().await?;
    Ok(tags)
}

pub async fn create(ipt: CreateTagInput) -> Result<Tag, ServiceError> {
    let tag = source::tag::create_tag(&ipt.name, &ipt.image).await?;
    Ok(tag)
}
