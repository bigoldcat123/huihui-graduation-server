use serde::{Deserialize, Serialize};

use crate::model::raw;


#[derive(Serialize,Deserialize)]
pub struct CurrentUser {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub profile: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct AuthResult {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub images: Option<Vec<String>>,
    pub create_at: String,
    pub location: String,
    pub is_public: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TopicUserInfo {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub profile: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TopicListItem {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub images: Option<Vec<String>>,
    pub create_at: String,
    pub location: String,
    pub is_public: bool,
    pub user_info: TopicUserInfo,
    pub comment_count: i64,
    pub like_count: i64,
    pub liked: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FoodTag {
    pub id: i32,
    pub name: String,
    pub image: String,
}

#[derive(Serialize, Deserialize)]
pub struct FoodWithTags {
    pub id: i32,
    pub restaurant_id: i32,
    pub restaurant_name: String,
    pub name: String,
    pub description: String,
    pub image: String,
    pub price: f64,
    pub tags: Vec<FoodTag>,
}

#[derive(Serialize, Deserialize)]
pub struct FoodSimple {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub price: f64,
}

#[derive(Serialize, Deserialize)]
pub struct FoodWithRestaurant {
    pub id: i32,
    pub restaurant_id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub price: f64,
    pub restaurant: Restaurant,
}

#[derive(Serialize, Deserialize)]
pub struct Restaurant {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub image: String,
}

#[derive(Serialize, Deserialize)]
pub struct RestaurantSimple {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct NameValue {
    pub name: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ReactionCount {
    pub like: i32,
    pub dislike: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Suggestion {
    pub id: i32,
    pub content: String,
    pub images: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub status: String,
    pub food: Option<FoodWithTags>,
    pub restaurant: Option<Restaurant>,
    pub reviewer_id: Option<i32>,
    pub review_comment: Option<String>,
    pub user_id: i32,
    pub created_at: String,
    pub reviewed_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TodoLogItem {
    pub content: String,
    pub create_time: String,
}

impl From<raw::User> for CurrentUser {
    fn from(user: raw::User) -> Self {
        CurrentUser {
            id: user.id,
            email: user.email,
            name: user.username,
            profile: user.profile,
        }
    }
}

impl From<raw::Topic> for Topic {
    fn from(topic: raw::Topic) -> Self {
        Topic {
            id: topic.id,
            user_id: topic.user_id,
            title: topic.title,
            content: topic.content,
            images: topic
                .images
                .and_then(|images| serde_json::from_str::<Vec<String>>(&images).ok()),
            create_at: topic.create_at.format("%Y-%m-%d").to_string(),
            location: topic.location,
            is_public: topic.is_public,
        }
    }
}

impl From<raw::TopicWithStats> for TopicListItem {
    fn from(topic: raw::TopicWithStats) -> Self {
        TopicListItem {
            id: topic.id,
            user_id: topic.user_id,
            title: topic.title,
            content: topic.content,
            images: topic
                .images
                .and_then(|images| serde_json::from_str::<Vec<String>>(&images).ok()),
            create_at: topic.create_at.format("%Y-%m-%d").to_string(),
            location: topic.location,
            is_public: topic.is_public,
            user_info: TopicUserInfo {
                id: topic.user_id,
                name: topic.user_name,
                email: topic.user_email,
                profile: topic.user_profile,
            },
            comment_count: topic.comment_count,
            like_count: topic.like_count,
            liked: topic.liked,
        }
    }
}

impl From<raw::Restaurant> for Restaurant {
    fn from(value: raw::Restaurant) -> Self {
        Restaurant {
            id: value.id,
            name: value.name,
            description: value.description,
            location: value.location,
            image: value.image,
        }
    }
}

impl From<raw::Restaurant> for RestaurantSimple {
    fn from(value: raw::Restaurant) -> Self {
        RestaurantSimple {
            id: value.id,
            name: value.name,
        }
    }
}

pub use crate::source::food_attribute::FoodAttributeRow as FoodAttribute;
