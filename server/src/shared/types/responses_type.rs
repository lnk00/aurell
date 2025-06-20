use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

pub struct ApiResponseWrapper<T>(pub ApiResponse<T>, pub StatusCode)
where
    T: Serialize;

impl<T> IntoResponse for ApiResponseWrapper<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (self.1, Json(self.0)).into_response()
    }
}

pub fn success_response<T: Serialize>(data: T) -> impl IntoResponse {
    ApiResponseWrapper(ApiResponse::success(data), StatusCode::OK)
}

pub fn error_response(message: String, status: StatusCode) -> impl IntoResponse {
    ApiResponseWrapper(ApiResponse::<()>::error(message), status)
}
