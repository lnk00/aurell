use futures::FutureExt;
use futures::future::BoxFuture;
use stytch::b2b::client::Client;
use stytch::b2b::magic_links_discovery::AuthenticateRequest;
use stytch::b2b::magic_links_email_discovery::SendRequest;

use crate::config::Config;
use crate::shared::errors::magic_link_service_errors::MagicLinkServiceError;
use aurell_types::auth::Org;

pub struct VerifyResult {
    pub token: String,
    pub orgs: Vec<Org>,
}

pub trait MagicLinkService {
    fn send(&self, email: String) -> BoxFuture<Result<(), MagicLinkServiceError>>;
    fn verify(&self, token: String) -> BoxFuture<Result<VerifyResult, MagicLinkServiceError>>;
}

pub struct StytchMagicLinkService {
    client: Client,
    config: Config,
}

impl StytchMagicLinkService {
    pub fn new(config: Config) -> Self {
        let client = Client::new(&config.stytch_project_id, &config.stytch_secret).unwrap();
        StytchMagicLinkService { client, config }
    }
}

impl MagicLinkService for StytchMagicLinkService {
    fn send(&self, email: String) -> BoxFuture<Result<(), MagicLinkServiceError>> {
        let redirect_url = format!(
            "{}{}",
            self.config.client_url, self.config.magic_link_redirect_url
        );

        let res = self.client.magic_links.email.discovery.send(SendRequest {
            email_address: email,
            discovery_redirect_url: Some(redirect_url),
            ..Default::default()
        });

        res.map(|result| match result {
            Ok(_) => Ok(()),
            Err(stytch::Error::Response(_)) => Err(MagicLinkServiceError::SendFailed),
            Err(_) => Err(MagicLinkServiceError::ServiceUnavailable),
        })
        .boxed()
    }

    fn verify(&self, token: String) -> BoxFuture<Result<VerifyResult, MagicLinkServiceError>> {
        let res = self
            .client
            .magic_links
            .discovery
            .authenticate(AuthenticateRequest {
                discovery_magic_links_token: token,
                pkce_code_verifier: None,
            });

        res.map(|result| match result {
            Ok(res) => Ok(VerifyResult {
                token: res.intermediate_session_token,
                orgs: res
                    .discovered_organizations
                    .into_iter()
                    .filter_map(|org| {
                        org.organization.map(|o| Org {
                            id: o.organization_id,
                            name: o.organization_name,
                        })
                    })
                    .collect(),
            }),
            Err(stytch::Error::Response(_)) => Err(MagicLinkServiceError::VerifyFailed),
            Err(_) => Err(MagicLinkServiceError::ServiceUnavailable),
        })
        .boxed()
    }
}
