use aurell_types::{ApiResponse, SendMagicLinkRequest, SendMagicLinkResponse};
use dioxus::logger::tracing::info;
use dioxus_query::prelude::MutationCapability;

use crate::http_client;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct SendMagicLinkMutation;

impl MutationCapability for SendMagicLinkMutation {
    type Ok = SendMagicLinkResponse;
    type Err = String;
    type Keys = SendMagicLinkRequest;

    async fn run(&self, args: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        info!("Sending magic link request for email: {}", args.email);

        let response = http_client::post::<SendMagicLinkRequest, SendMagicLinkResponse>(
            "/auth/magiclink/send",
            args,
        )
        .await;

        match response {
            ApiResponse {
                success: true,
                data: Some(data),
                ..
            } => Ok(data),
            ApiResponse {
                success: false,
                error: Some(error),
                ..
            } => Err(error),
            _ => {
                let error = "Unknown error occurred".to_string();
                Err(error)
            }
        }
    }
}
