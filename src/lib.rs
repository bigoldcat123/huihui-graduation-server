use std::sync::OnceLock;

use sqlx::{Pool, Postgres};

pub mod handlers;
pub mod model;
pub mod source;
pub mod service;
pub static DB: OnceLock<Pool<Postgres>> = OnceLock::new();
pub const ROOT_USER_ID: i32 = 6;

pub async fn init_db() {
    let pool = Pool::connect("postgres://admin:root@localhost:5432/huihui")
        .await
        .expect("URL GG");
    DB.set(pool).expect("msg");
}

pub async fn init_db_if_not() {
    if DB.get().is_none() {
        init_db().await;
    }
}

pub fn db() -> &'static Pool<Postgres> {
    DB.get().expect("database not initialized")
}
