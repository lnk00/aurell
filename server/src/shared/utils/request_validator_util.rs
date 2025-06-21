use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::responses_util::error_response;

pub fn validate_request<T>(payload: Result<Json<T>, JsonRejection>) -> Result<T, Response> {
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
