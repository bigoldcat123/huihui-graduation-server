use std::sync::OnceLock;

use sqlx::{Pool, Postgres};

pub mod handlers;
pub mod model;
pub mod source;
pub mod service;
pub static DB:OnceLock<Pool<Postgres>> = OnceLock::new();

pub async fn init_db() {
    DB.set(Pool::connect("postgres://7ec6646b2b2c6517bfeb7823d6db9df0431b3f5ecb1c3a9fe842e9b8d6e69b74:sk_3wxpCZ5W7LL1Z_XOBUywc@db.prisma.io:5432/huihui?sslmode=require").await.expect("URL GG")).expect("msg");
}
pub fn db() -> &'static Pool<Postgres> {
    DB.get().expect("database not initialized")
}
