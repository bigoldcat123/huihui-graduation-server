use faithea::post;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
struct Input {
    url:String
}

#[post("/")]
async fn food_recog() {
    ""
}
