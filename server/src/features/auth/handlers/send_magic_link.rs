use crate::{
    features::auth::services::ServiceContainer,
    shared::types::responses_type::{error_response, success_response},
};
use aurell_types::auth::{SendMagicLinkRequest, SendMagicLinkResponse};
use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn handle(
    State(sc): State<ServiceContainer>,
    payload: Result<Json<SendMagicLinkRequest>, JsonRejection>,
) -> Response {
    let request = match validate_request(payload) {
        Ok(req) => req,
        Err(response) => return response,
    };

    match sc.magic_link_service.send(request.email).await {
        Err(e) => error_response(e.to_string(), StatusCode::BAD_REQUEST).into_response(),
        Ok(_) => success_response(SendMagicLinkResponse {}).into_response(),
    }
}

fn validate_request(
    payload: Result<Json<SendMagicLinkRequest>, JsonRejection>,
) -> Result<SendMagicLinkRequest, Response> {
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
