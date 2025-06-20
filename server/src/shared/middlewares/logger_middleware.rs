use axum::{body::Body, http::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn logger_middleware(req: Request<Body>, next: Next) -> Response {
    info!("{} {}", req.method(), req.uri().path());
    next.run(req).await
}
