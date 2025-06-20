use std::sync::Arc;

use crate::config::Config;
use magic_link_service::{MagicLinkService, StytchMagicLinkService};
use session_service::{SessionService, StytchSessionService};

pub mod magic_link_service;
pub mod session_service;

#[derive(Clone)]
pub struct ServiceContainer {
    pub session_service: Arc<dyn SessionService + Send + Sync>,
    pub magic_link_service: Arc<dyn MagicLinkService + Send + Sync>,
}

impl ServiceContainer {
    pub fn new(config: Config) -> Self {
        Self {
            session_service: Arc::new(StytchSessionService::new(config.clone())),
            magic_link_service: Arc::new(StytchMagicLinkService::new(config.clone())),
        }
    }
}
