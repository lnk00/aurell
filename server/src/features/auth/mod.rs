use crate::config::Config;
use axum::{
    Router,
    routing::{get, post},
};
use handlers::{revoke_session, send_magic_link, signin, verify_magic_link};
use services::ServiceContainer;

pub mod handlers;
pub mod services;

pub struct AuthHandlers;

impl AuthHandlers {
    pub fn routes(config: Config) -> Router {
        let service_container = ServiceContainer::new(config);

        Router::new()
            .route("/auth/session/revoke", get(revoke_session::handle))
            .route("/auth/magiclink/send", post(send_magic_link::handle))
            .route("/auth/magiclink/verify", post(verify_magic_link::handle))
            .route("/auth/magiclink/signin", post(signin::handle))
            .with_state(service_container)
    }
}
