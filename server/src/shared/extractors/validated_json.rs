use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;

use crate::shared::utils::responses_util::error_response;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(payload)) => Ok(ValidatedJson(payload)),
            Err(rejection) => {
                let error_msg = format!("Invalid JSON payload: {}", rejection);
                Err(error_response(error_msg, StatusCode::BAD_REQUEST).into_response())
            }
        }
    }
}

// Implement Deref so you can access the inner value directly
impl<T> std::ops::Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implement DerefMut for mutable access
impl<T> std::ops::DerefMut for ValidatedJson<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
