use faithea::{handlers, server::HandlerModifier};

pub mod auth;
pub mod food;
pub mod static_files;


pub fn auth_handlers() -> Vec<HandlerModifier> {
    handlers!(auth::login, auth::register, auth::me)
}

pub fn food_handlers() -> Vec<HandlerModifier> {
    handlers!(food::recommendation, food::recommendation_reaction, food::consecutive_suggest)
}

pub fn static_handlers() -> Vec<HandlerModifier> {
    handlers!(static_files::serve_static)
}
