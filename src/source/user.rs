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

pub async fn create_user(email: &str, username: &str, password: &str) -> Result<User, sqlx::Error> {
    let user: User = sqlx::query_as(
        "INSERT INTO _user (email, username, password) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(email)
    .bind(username)
    .bind(password)
    .fetch_one(db())
    .await?;
    Ok(user)
}

pub async fn update_user_info(
    user_id: i32,
    email: Option<&str>,
    username: Option<&str>,
    profile: Option<&str>,
) -> Result<User, sqlx::Error> {
    let user: User = sqlx::query_as(
        r#"
        UPDATE _user
        SET
            email = COALESCE($2, email),
            username = COALESCE($3, username),
            profile = COALESCE($4, profile)
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(email)
    .bind(username)
    .bind(profile)
    .fetch_one(db())
    .await?;
    Ok(user)
}
