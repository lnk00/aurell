use crate::http_client::post;
use aurell_types::{ApiResponse, SendMagicLinkRequest, SendMagicLinkResponse};
use dioxus_query::prelude::MutationCapability;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct SendMagicLinkMutation;

impl MutationCapability for SendMagicLinkMutation {
    type Ok = SendMagicLinkResponse;
    type Err = String;
    type Keys = SendMagicLinkRequest;

    async fn run(&self, args: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let res = post::<Self::Keys, Self::Ok>("/auth/magiclink/send", args).await;

        match res {
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
            _ => Err("Unknown error occurred".to_string()),
        }
    }
}
