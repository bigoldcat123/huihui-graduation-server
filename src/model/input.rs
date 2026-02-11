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
#[derive(Serialize,Deserialize,Debug)]
pub struct SuggestionInput{
    pub food_ids: Vec<i32>,
    pub selected_food_ids: Vec<i32>,
}
