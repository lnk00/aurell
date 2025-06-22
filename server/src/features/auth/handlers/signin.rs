use crate::{
    features::auth::services::ServiceContainer,
    shared::extractors::validated_json::ValidatedJson,
    shared::utils::responses_util::{error_response, success_response},
};
use aurell_types::auth::{SigninRequest, SigninResponse};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn handle(
    State(sc): State<ServiceContainer>,
    ValidatedJson(request): ValidatedJson<SigninRequest>,
) -> Response {
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
