use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub profile: Option<String>,
    pub password: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub image: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Topic {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub images: Option<String>,
    pub create_at: DateTime<Local>,
}

#[derive(Debug, Clone, FromRow)]
pub struct TopicWithStats {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub images: Option<String>,
    pub create_at: DateTime<Local>,
    pub user_name: String,
    pub user_email: String,
    pub user_profile: Option<String>,
    pub comment_count: i64,
    pub like_count: i64,
    pub liked: bool,
}
