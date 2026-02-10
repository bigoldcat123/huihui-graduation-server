use crate::{db, model::raw::User};

pub async fn get_user_by_id(id: i32) -> Result<User, sqlx::Error> {
    let user: User = sqlx::query_as("SELECT * FROM _user WHERE id = $1")
        .bind(id)
        .fetch_one(db())
        .await?;
    Ok(user)
}

pub async fn get_user_by_name(username: &str) -> Result<User, sqlx::Error> {
    let user: User = sqlx::query_as("SELECT * FROM _user WHERE username = $1")
        .bind(username)
        .fetch_one(db())
        .await?;
    Ok(user)
}

pub async fn get_user_by_email(email: &str) -> Result<User, sqlx::Error> {
    let user: User = sqlx::query_as("SELECT * FROM _user WHERE email = $1")
        .bind(email)
        .fetch_one(db())
        .await?;
    Ok(user)
}
