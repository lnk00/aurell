use crate::{
    features::auth::services::ServiceContainer,
    shared::utils::{
        request_validator_util::validate_request,
        responses_util::{error_response, success_response},
    },
};
use aurell_types::auth::{VerifyMagicLinkRequest, VerifyMagicLinkResponse};
use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn handle(
    State(sc): State<ServiceContainer>,
    payload: Result<Json<VerifyMagicLinkRequest>, JsonRejection>,
) -> Response {
    let request = match validate_request(payload) {
        Ok(req) => req,
        Err(response) => return response,
    };

    match sc.magic_link_service.verify(request.token).await {
        Err(e) => error_response(e.to_string(), StatusCode::BAD_REQUEST).into_response(),
        Ok(res) => success_response(VerifyMagicLinkResponse {
            token: res.token,
            orgs: res.orgs,
        })
        .into_response(),
    }
}
