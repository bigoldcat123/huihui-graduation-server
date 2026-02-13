use crate::{
    model::{input::CreateTopicInput, output::{Topic, TopicListItem}},
    service::error::ServiceError,
    source,
};

const PAGE_SIZE: i64 = 10;

pub async fn list(page: Option<i64>) -> Result<Vec<TopicListItem>, ServiceError> {
    let page = page.unwrap_or(1);
    let raw_topics = source::topic::list_topics_by_page(page, PAGE_SIZE).await?;
    Ok(raw_topics.into_iter().map(TopicListItem::from).collect())
}

pub async fn create(user_id: i32, ipt: CreateTopicInput) -> Result<(), ServiceError> {
    let is_top = ipt.reply_to_id.is_none();
    let images_json = ipt
        .images
        .as_ref()
        .map(serde_json::to_string)
        .transpose()?;
    let topic = source::topic::create_topic(
        user_id,
        &ipt.title,
        &ipt.content,
        is_top,
        images_json.as_deref(),
    )
    .await?;

    if let Some(reply_to_id) = ipt.reply_to_id {
        source::topic::create_reply(topic.id, reply_to_id).await?;
    }

    Ok(())
}
