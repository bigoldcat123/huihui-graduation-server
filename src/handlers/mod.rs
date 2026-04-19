use faithea::{handlers, server::HandlerModifier};

pub mod auth;
pub mod food;
pub mod food_comment;
pub mod static_files;
pub mod upload;
pub mod topic;
pub mod tag;
pub mod restaurant;
pub mod suggestion;
pub mod user_profile;
pub mod user_calorie_goal;
pub mod meal_record;


pub fn auth_handlers() -> Vec<HandlerModifier> {
    handlers!(auth::login, auth::root_login, auth::register, auth::me, auth::update_user_info)
}

pub fn food_handlers() -> Vec<HandlerModifier> {
    handlers!(
        food::recommendation,
        food::list_liked_foods,
        food::recommendation_reaction,
        food::reaction_count,
        food::top_liked_tags,
        food::consecutive_suggest,
        food::list_foods,
        food::create_food,
        food::update_food,
        food::get_food_attribute
    )
}

pub fn food_comment_handlers() -> Vec<HandlerModifier> {
    handlers!(
        food_comment::get_food_comments,
        food_comment::create_food_comment,
        food_comment::toggle_comment_thumb
    )
}

pub fn static_handlers() -> Vec<HandlerModifier> {
    handlers!(static_files::serve_static)
}

pub fn upload_handlers() -> Vec<HandlerModifier> {
    handlers!(upload::upload)
}

pub fn topic_handlers() -> Vec<HandlerModifier> {
    handlers!(
        topic::list_topics,
        topic::create_topic,
        topic::like_topic,
        topic::list_comment,
        topic::list_my_topics,
        topic::delete_topic
    )
}

pub fn tag_handlers() -> Vec<HandlerModifier> {
    handlers!(tag::list_tags, tag::create_tag, tag::list_liked_values)
}

pub fn user_profile_handlers() -> Vec<HandlerModifier> {
    handlers!(user_profile::get, user_profile::update)
}

pub fn user_calorie_goal_handlers() -> Vec<HandlerModifier> {
    handlers!(user_calorie_goal::get, user_calorie_goal::set)
}

pub fn meal_record_handlers() -> Vec<HandlerModifier> {
    handlers!(meal_record::get_today, meal_record::create, meal_record::insert_from_inner_food)
}

pub fn restaurant_handlers() -> Vec<HandlerModifier> {
    handlers!(
        restaurant::list_restaurants,
        restaurant::list_restaurants_simple,
        restaurant::list_foods_by_restaurant_id,
        restaurant::list_restaurants_by_page,
        restaurant::create_restaurant,
        restaurant::update_restaurant
    )
}

pub fn suggestion_handlers() -> Vec<HandlerModifier> {
    handlers!(
        suggestion::create_suggestion,
        suggestion::list_my_suggestion,
        suggestion::get_suggestion_by_id,
        suggestion::list_todo_logs_by_suggestion_status,
        suggestion::list_suggestion_by_page,
        suggestion::list_todos_by_page,
        suggestion::review_suggestion,
        suggestion::move_suggestion_to_next_stage,
        suggestion::add_todo_log
    )
}
