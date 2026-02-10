use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
pub struct UsernamePasswordAuthentication {
    pub username: String,
    pub password: String,
}
