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

#[derive(Serialize, Deserialize)]
pub struct FoodComment {
    pub id: i32,
    pub food_id: i32,
    pub user_id: i32,
    pub content: String,
    pub create_time: String,
    pub thumb_count: i64,
    pub thumbed: bool,
}

impl From<raw::FoodCommentRow> for FoodComment {
    fn from(row: raw::FoodCommentRow) -> Self {
        FoodComment {
            id: row.id,
            food_id: row.food_id,
            user_id: row.user_id,
            content: row.content,
            create_time: row.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            thumb_count: row.thumb_count,
            thumbed: row.thumbed,
        }
    }
}

impl From<raw::UserProfile> for UserProfileOutput {
    fn from(row: raw::UserProfile) -> Self {
        UserProfileOutput {
            id: row.id,
            user_id: row.user_id,
            height_cm: row.height_cm,
            weight_kg: row.weight_kg,
            birth_date: row.birth_date.map(|d| d.format("%Y-%m-%d").to_string()),
            gender: row.gender,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserProfileOutput {
    pub id: i32,
    pub user_id: i32,
    pub height_cm: f64,
    pub weight_kg: f64,
    pub birth_date: Option<String>,
    pub gender: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserCalorieGoalOutput {
    pub id: i32,
    pub user_id: i32,
    pub daily_calorie_goal: f32,
    pub effective_from: String,
    pub effective_to: Option<String>,
}

impl From<raw::UserCalorieGoal> for UserCalorieGoalOutput {
    fn from(row: raw::UserCalorieGoal) -> Self {
        UserCalorieGoalOutput {
            id: row.id,
            user_id: row.user_id,
            daily_calorie_goal: row.daily_calorie_goal,
            effective_from: row.effective_from.format("%Y-%m-%d").to_string(),
            effective_to: row.effective_to.map(|d| d.format("%Y-%m-%d").to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MealRecordOutput {
    pub id: i32,
    pub user_id: i32,
    pub meal_type: String,
    pub source_type: String,
    pub total_calories: f32,
    pub note: Option<String>,
    pub created_at: String,
}

impl From<raw::MealRecord> for MealRecordOutput {
    fn from(row: raw::MealRecord) -> Self {
        MealRecordOutput {
            id: row.id,
            user_id: row.user_id,
            meal_type: row.meal_type,
            source_type: row.source_type,
            total_calories: row.total_calories,
            note: row.note,
            created_at: row.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ExerciseTypeOutput {
    pub id: i32,
    pub name: String,
    pub met_value: f32,
}

impl From<raw::ExerciseType> for ExerciseTypeOutput {
    fn from(row: raw::ExerciseType) -> Self {
        ExerciseTypeOutput {
            id: row.id,
            name: row.name,
            met_value: row.met_value,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ExerciseRecordOutput {
    pub id: i32,
    pub user_id: i32,
    pub exercise_type_id: i32,
    pub exercise_name_snapshot: String,
    pub met_value_snapshot: f32,
    pub duration_minutes: i32,
    pub body_weight_kg: f32,
    pub calories_burned: f32,
    pub occurred_at: String,
    pub created_at: String,
}

impl From<raw::ExerciseRecord> for ExerciseRecordOutput {
    fn from(row: raw::ExerciseRecord) -> Self {
        ExerciseRecordOutput {
            id: row.id,
            user_id: row.user_id,
            exercise_type_id: row.exercise_type_id,
            exercise_name_snapshot: row.exercise_name_snapshot,
            met_value_snapshot: row.met_value_snapshot,
            duration_minutes: row.duration_minutes,
            body_weight_kg: row.body_weight_kg,
            calories_burned: row.calories_burned,
            occurred_at: row.occurred_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            created_at: row.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
