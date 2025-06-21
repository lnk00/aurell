use crate::{
    features::auth::services::ServiceContainer,
    shared::utils::{
        request_validator_util::validate_request,
        responses_util::{error_response, success_response},
    },
};
use aurell_types::auth::{RevokeSessionRequest, RevokeSessionResponse};
use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn handle(
    State(sc): State<ServiceContainer>,
    payload: Result<Json<RevokeSessionRequest>, JsonRejection>,
) -> Response {
    let request = match validate_request(payload) {
        Ok(req) => req,
        Err(response) => return response,
    };

    match sc.session_service.revoke(request.session_token).await {
        Err(e) => error_response(e.to_string(), StatusCode::BAD_REQUEST).into_response(),
        Ok(_) => success_response(RevokeSessionResponse {}).into_response(),
    }
}
