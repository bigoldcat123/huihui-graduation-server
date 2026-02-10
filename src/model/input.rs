use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
pub struct UsernamePasswordAuthentication {
    pub username: String,
    pub password: String,
}

#[derive(Serialize,Deserialize)]
pub struct RegisterInput {
    pub email: String,
    pub username: String,
    pub password: String,
}
