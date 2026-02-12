use faithea::{get, util};

#[get("/**")]
async fn serve_static() {
    util::static_map(_req, "static").await
}
