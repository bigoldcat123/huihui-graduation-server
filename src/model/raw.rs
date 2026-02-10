use chrono::{DateTime, Local};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub image: String,
}
