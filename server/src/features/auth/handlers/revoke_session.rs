use crate::{
    features::auth::services::ServiceContainer,
    shared::types::responses_type::{error_response, success_response},
};
use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RevokeSessionRequest {
    pub session_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct RevokeSessionResponse {}

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

fn validate_request(
    payload: Result<Json<RevokeSessionRequest>, JsonRejection>,
) -> Result<RevokeSessionRequest, Response> {
    let request = match payload {
        Ok(Json(req)) => req,
        Err(rejection) => {
            return Err(error_response(
                format!("Invalid JSON payload: {}", rejection),
                StatusCode::BAD_REQUEST,
            )
            .into_response());
        }
    };

    Ok(request)
}
