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

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserInfoInput {
    pub email: Option<String>,
    pub username: Option<String>,
    pub profile: Option<String>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFoodInput {
    pub restaurant_id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub tag_ids: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTagInput {
    pub name: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRestaurantInput {
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRestaurantInput {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFoodInput {
    pub id: i32,
    pub restaurant_id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub tag_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SuggestionType {
    AddFood,
    UpdateFood,
    Other,
}

impl SuggestionType {
    pub fn as_db_str(self) -> &'static str {
        match self {
            SuggestionType::AddFood => "ADD_FOOD",
            SuggestionType::UpdateFood => "UPDATE_FOOD",
            SuggestionType::Other => "OTHER",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSuggestionInput {
    pub content: String,
    pub images: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: SuggestionType,
    pub food_id: Option<i32>,
    pub restaurant_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SuggestionReviewStatus {
    Approved,
    Rejected,
}

impl SuggestionReviewStatus {
    pub fn as_db_str(self) -> &'static str {
        match self {
            SuggestionReviewStatus::Approved => "APPROVED",
            SuggestionReviewStatus::Rejected => "REJECTED",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReviewSuggestionInput {
    pub suggestion_id: i32,
    pub status: SuggestionReviewStatus,
    pub review_comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveSuggestionNextInput {
    pub suggestion_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddTodoLogInput {
    pub suggestion_id: i32,
    pub current_status: String,
    pub log_content: String,
}
