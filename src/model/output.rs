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
    pub images: Option<String>,
    pub create_at: String,
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
            images: topic.images,
            create_at: topic.create_at.to_rfc3339(),
        }
    }
}
