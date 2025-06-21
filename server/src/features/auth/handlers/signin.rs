use crate::{
    features::auth::services::ServiceContainer,
    shared::utils::request_validator_util::validate_request,
    shared::utils::responses_util::{error_response, success_response},
};
use aurell_types::auth::{SigninRequest, SigninResponse};
use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn handle(
    State(sc): State<ServiceContainer>,
    payload: Result<Json<SigninRequest>, JsonRejection>,
) -> Response {
    let request = match validate_request(payload) {
        Ok(req) => req,
        Err(response) => return response,
    };

    match sc
        .session_service
        .signin(request.org_id, request.token)
        .await
    {
        Err(e) => error_response(e.to_string(), StatusCode::BAD_REQUEST).into_response(),
        Ok(res) => success_response(SigninResponse {
            session_token: res.session_token,
            session_jwt: res.session_jwt,
        })
        .into_response(),
    }
}
