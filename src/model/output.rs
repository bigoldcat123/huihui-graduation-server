use serde::{Deserialize, Serialize};

use crate::model::raw;


#[derive(Serialize,Deserialize)]
pub struct CurrentUser {
    pub id: i32,
    pub email: String,
    pub name: String,
}

#[derive(Serialize,Deserialize)]
pub struct AuthResult {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub images: Option<Vec<String>>,
    pub create_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopicUserInfo {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopicListItem {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub images: Option<Vec<String>>,
    pub create_at: String,
    pub user_info: TopicUserInfo,
    pub comment_count: i64,
    pub like_count: i64,
}

impl From<raw::User> for CurrentUser {
    fn from(user: raw::User) -> Self {
        CurrentUser {
            id: user.id,
            email: user.email,
            name: user.username,
        }
    }
}

impl From<raw::Topic> for Topic {
    fn from(topic: raw::Topic) -> Self {
        Topic {
            id: topic.id,
            user_id: topic.user_id,
            title: topic.title,
            content: topic.content,
            images: topic
                .images
                .and_then(|images| serde_json::from_str::<Vec<String>>(&images).ok()),
            create_at: topic.create_at.to_rfc3339(),
        }
    }
}

impl From<raw::TopicWithStats> for TopicListItem {
    fn from(topic: raw::TopicWithStats) -> Self {
        TopicListItem {
            id: topic.id,
            user_id: topic.user_id,
            title: topic.title,
            content: topic.content,
            images: topic
                .images
                .and_then(|images| serde_json::from_str::<Vec<String>>(&images).ok()),
            create_at: topic.create_at.to_rfc3339(),
            user_info: TopicUserInfo {
                id: topic.user_id,
                name: topic.user_name,
                email: topic.user_email,
            },
            comment_count: topic.comment_count,
            like_count: topic.like_count,
        }
    }
}
