use axum::{http::StatusCode, response::IntoResponse};

pub async fn health_check() -> impl IntoResponse {
(StatusCode::OK, "ALL RIGHT!, I'am Good").into_response()
}