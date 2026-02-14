use crate::{model::raw::Tag, service::error::ServiceError, source};

pub async fn list() -> Result<Vec<Tag>, ServiceError> {
    let tags = source::tag::list_tags().await?;
    Ok(tags)
}
