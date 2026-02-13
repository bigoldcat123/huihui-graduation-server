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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Reaction {
    Like,
    Skip,
    Dislike,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendationReactionInput {
    pub food_id: i32,
    pub reaction: Reaction,
    pub source: String,
    pub occurred_at: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTopicInput {
    pub title: String,
    pub content: String,
    pub images: Option<Vec<String>>,
    pub reply_to_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicLikeInput {
    pub topic_id: i32,
    pub like: bool,
}
