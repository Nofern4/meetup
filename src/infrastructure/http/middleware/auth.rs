use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::{Response, IntoResponse},
};

use crate::config::config_loader::get_jwt_env;

pub async fn authorization(mut req: Request, next: Next) -> Result<Response, Response> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing or invalid Authorization header").into_response())?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Invalid Bearer token format").into_response())?;

    let secret_env = get_jwt_env()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get JWT secret: {}", e)).into_response())?;

    let claims = crate::infrastructure::jwt::verify_token(secret_env, token.to_string())
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Token verification failed: {}", e)).into_response())?;

    let brawler_id = claims
        .sub
        .parse::<i32>()
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid subject in token: {}", e)).into_response())?;

    req.extensions_mut().insert(brawler_id);

    Ok(next.run(req).await)
}
