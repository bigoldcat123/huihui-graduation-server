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
pub struct Restaurant {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub location: String,
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

#[derive(Debug, Clone, FromRow)]
pub struct Suggestion {
    pub id: i32,
    pub content: String,
    pub images: Option<String>,
    pub r#type: String,
    pub status: String,
    pub food_id: Option<i32>,
    pub restaurant_id: Option<i32>,
    pub reviewer_id: Option<i32>,
    pub review_comment: Option<String>,
    pub user_id: i32,
    pub created_at: DateTime<Local>,
    pub reviewed_at: Option<DateTime<Local>>,
}
