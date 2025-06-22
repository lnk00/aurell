use crate::{
    features::auth::services::ServiceContainer,
    shared::extractors::validated_json::ValidatedJson,
    shared::utils::responses_util::{error_response, success_response},
};
use aurell_types::auth::{RevokeSessionRequest, RevokeSessionResponse};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn handle(
    State(sc): State<ServiceContainer>,
    ValidatedJson(request): ValidatedJson<RevokeSessionRequest>,
) -> Response {
    match sc.session_service.revoke(request.session_token).await {
        Err(e) => error_response(e.to_string(), StatusCode::BAD_REQUEST).into_response(),
        Ok(_) => success_response(RevokeSessionResponse {}).into_response(),
    }
}
