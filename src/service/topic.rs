use crate::{
    model::{input::{CreateTopicInput, TopicLikeInput}, output::TopicListItem},
    service::error::ServiceError,
    source,
};

const PAGE_SIZE: i64 = 10;

pub async fn list(page: Option<i64>, user_id: i32) -> Result<Vec<TopicListItem>, ServiceError> {
    let page = page.unwrap_or(1);
    let raw_topics = source::topic::list_topics_by_page(page, PAGE_SIZE, user_id).await?;
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
        &ipt.location,
        ipt.is_public,
    )
    .await?;

    if let Some(reply_to_id) = ipt.reply_to_id {
        source::topic::create_reply(topic.id, reply_to_id).await?;
    }

    Ok(())
}

pub async fn set_like(user_id: i32, ipt: TopicLikeInput) -> Result<(), ServiceError> {
    if ipt.like {
        source::topic_like::like_topic(user_id, ipt.topic_id).await?;
    } else {
        source::topic_like::unlike_topic(user_id, ipt.topic_id).await?;
    }
    Ok(())
}

pub async fn list_comment(topic_id: i32, user_id: i32) -> Result<Vec<TopicListItem>, ServiceError> {
    let raw_topics = source::topic::list_comments_by_topic_id(topic_id, user_id).await?;
    Ok(raw_topics.into_iter().map(TopicListItem::from).collect())
}

pub async fn list_my_topics(user_id: i32) -> Result<Vec<TopicListItem>, ServiceError> {
    let raw_topics = source::topic::list_topics_by_user_id(user_id, user_id).await?;
    Ok(raw_topics.into_iter().map(TopicListItem::from).collect())
}

pub async fn delete(topic_id: i32, user_id: i32) -> Result<(), ServiceError> {
    source::topic::ensure_topic_owner(topic_id, user_id).await?;
    source::topic::delete_topic_with_comments(topic_id).await?;
    Ok(())
}
