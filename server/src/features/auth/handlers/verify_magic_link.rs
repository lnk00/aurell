use crate::{
    features::auth::services::ServiceContainer,
    shared::extractors::validated_json::ValidatedJson,
    shared::utils::responses_util::{error_response, success_response},
};
use aurell_types::auth::{VerifyMagicLinkRequest, VerifyMagicLinkResponse};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn handle(
    State(sc): State<ServiceContainer>,
    ValidatedJson(request): ValidatedJson<VerifyMagicLinkRequest>,
) -> Response {
    match sc.magic_link_service.verify(request.token).await {
        Err(e) => error_response(e.to_string(), StatusCode::BAD_REQUEST).into_response(),
        Ok(res) => success_response(VerifyMagicLinkResponse {
            token: res.token,
            orgs: res.orgs,
        })
        .into_response(),
    }
}
