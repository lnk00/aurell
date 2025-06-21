use crate::{
    features::auth::services::ServiceContainer,
    shared::types::responses_type::{error_response, success_response},
};
use aurell_shared::auth::{VerifyMagicLinkRequest, VerifyMagicLinkResponse};
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

fn validate_request(
    payload: Result<Json<VerifyMagicLinkRequest>, JsonRejection>,
) -> Result<VerifyMagicLinkRequest, Response> {
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
