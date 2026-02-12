use faithea::{get, handlers, server::HttpServer};
use huihui_server::{handlers::{auth_handlers, food_handlers, static_handlers, upload_handlers}, init_db};

#[get("/")]
async fn hello() {
    "hello"
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();
    init_db().await;
    let _ = HttpServer::builder()
        .mount("/", handlers!(hello))
        .mount("/", upload_handlers())
        .mount("/auth", auth_handlers())
        .mount("/food", food_handlers())
        .mount("/static", static_handlers())
        .host("0.0.0.0")
        .build()
        .run()
        .await;
}
