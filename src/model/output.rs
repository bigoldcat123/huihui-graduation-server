use serde::{Deserialize, Serialize};

use crate::model::raw;


#[derive(Serialize,Deserialize)]
pub struct CurrentUser {
    pub id: i32,
    pub email: String,
    pub name: String,
}

#[derive(Serialize,Deserialize)]
pub struct AuthResult {
    pub token: String,
}
impl From<raw::User> for CurrentUser {
    fn from(user: raw::User) -> Self {
        CurrentUser {
            id: user.id,
            email: user.email,
            name: user.username,
        }
    }
}
