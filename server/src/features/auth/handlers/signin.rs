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
pub struct SigninRequest {
    pub token: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct SigninResponse {
    #[serde(rename = "sessionToken")]
    pub session_token: String,
    #[serde(rename = "sessionJwt")]
    pub session_jwt: String,
}

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

fn validate_request(
    payload: Result<Json<SigninRequest>, JsonRejection>,
) -> Result<SigninRequest, Response> {
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
