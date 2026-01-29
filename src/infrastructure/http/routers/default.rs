use axum::{Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}

pub async fn make_error(Path(code): Path<u16>) -> impl IntoResponse {
    let status_code = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status_code, code.to_string()).into_response()
}

pub fn routes() -> Router {
    Router::new()
        .route("/health-check", get(health_check))
        .route("/make-error/{code}", get(make_error))
}