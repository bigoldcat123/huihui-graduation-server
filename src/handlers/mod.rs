use faithea::{handlers, server::HandlerModifier};

pub mod auth;


pub fn auth_handlers() -> Vec<HandlerModifier> {
    handlers!(auth::login)
}
