use crate::{
    model::output::Topic,
    service::error::ServiceError,
    source,
};

const PAGE_SIZE: i64 = 10;

pub async fn list(page: Option<i64>) -> Result<Vec<Topic>, ServiceError> {
    let page = page.unwrap_or(1);
    let raw_topics = source::topic::list_topics_by_page(page, PAGE_SIZE).await?;
    Ok(raw_topics.into_iter().map(Topic::from).collect())
}
