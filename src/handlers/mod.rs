use faithea::{handlers, server::HandlerModifier};

pub mod auth;
pub mod food;
pub mod static_files;
pub mod upload;
pub mod topic;
pub mod tag;


pub fn auth_handlers() -> Vec<HandlerModifier> {
    handlers!(auth::login, auth::root_login, auth::register, auth::me, auth::update_user_info)
}

pub fn food_handlers() -> Vec<HandlerModifier> {
    handlers!(food::recommendation, food::recommendation_reaction, food::consecutive_suggest, food::list_foods)
}

pub fn static_handlers() -> Vec<HandlerModifier> {
    handlers!(static_files::serve_static)
}

pub fn upload_handlers() -> Vec<HandlerModifier> {
    handlers!(upload::upload)
}

pub fn topic_handlers() -> Vec<HandlerModifier> {
    handlers!(topic::list_topics, topic::create_topic, topic::like_topic, topic::list_comment)
}

pub fn tag_handlers() -> Vec<HandlerModifier> {
    handlers!(tag::list_tags)
}
