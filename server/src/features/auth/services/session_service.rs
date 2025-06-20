use futures::FutureExt;
use futures::future::BoxFuture;
use stytch::b2b::{
    client::Client, discovery_intermediate_sessions::ExchangeRequest, sessions::RevokeRequest,
};

use crate::{config::Config, shared::errors::session_service_errors::SessionServiceError};

pub struct SigninResult {
    pub session_token: String,
    pub session_jwt: String,
}

pub trait SessionService {
    fn revoke(&self, session_token: String) -> BoxFuture<Result<(), SessionServiceError>>;
    fn signin(
        &self,
        org_id: String,
        token: String,
    ) -> BoxFuture<Result<SigninResult, SessionServiceError>>;
}

pub struct StytchSessionService {
    client: Client,
}

impl StytchSessionService {
    pub fn new(config: Config) -> Self {
        let client = Client::new(&config.stytch_project_id, &config.stytch_secret).unwrap();
        StytchSessionService { client }
    }
}

impl SessionService for StytchSessionService {
    fn revoke(&self, session_token: String) -> BoxFuture<Result<(), SessionServiceError>> {
        let res = self.client.sessions.revoke(RevokeRequest {
            session_token: Some(session_token),
            member_session_id: None,
            session_jwt: None,
            member_id: None,
        });

        res.map(|result| match result {
            Ok(_) => Ok(()),
            Err(stytch::Error::Response(_)) => Err(SessionServiceError::RevokeFailed),
            Err(_) => Err(SessionServiceError::ServiceUnavailable),
        })
        .boxed()
    }

    fn signin(
        &self,
        org_id: String,
        token: String,
    ) -> BoxFuture<Result<SigninResult, SessionServiceError>> {
        let res = self
            .client
            .discovery
            .intermediate_sessions
            .exchange(ExchangeRequest {
                intermediate_session_token: token,
                organization_id: org_id,
                session_duration_minutes: Some(60),
                session_custom_claims: None,
                locale: None,
            });

        res.map(|result| match result {
            Ok(res) => Ok(SigninResult {
                session_token: res.session_token,
                session_jwt: res.session_jwt,
            }),
            Err(stytch::Error::Response(_)) => Err(SessionServiceError::SigninFailed),
            Err(_) => Err(SessionServiceError::ServiceUnavailable),
        })
        .boxed()
    }
}
