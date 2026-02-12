use faithea::{handlers, server::HandlerModifier};

pub mod auth;
pub mod food;


pub fn auth_handlers() -> Vec<HandlerModifier> {
    handlers!(auth::login, auth::register, auth::me)
}

pub fn food_handlers() -> Vec<HandlerModifier> {
    handlers!(food::recommendation, food::recommendation_reaction, food::consecutive_suggest)
}
