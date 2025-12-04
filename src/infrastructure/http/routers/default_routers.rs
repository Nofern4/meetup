use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode, header},
    middleware::Next,
};

use crate::{application::use_cases::authentication::get_user_secret_env, infrastructure};

pub async fn auth() {}

fn get_cookie_value(cookie_header: &str, key: &str) -> Option<String> {
    cookie_header.split("; ").find_map(|cookie| {
        let mut parts = cookie.splitn(2, '=');
        let name = parts.next()?.trim();
        let value = parts.next()?.trim();
        if name == key {
            Some(value.to_string())
        } else {
            None
        }
    })
}


pub async fn authorizaton(
    mut req: Request,
    next: Next
) -> std::result::Result<Response<Body>, StatusCode> {
    let cookie_str = req
        .headers()
        .get(header::COOKIE)
        .and_then(|cookie_header| cookie_header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = get_cookie_value(cookie_str, "auth_token")
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let secret_env = get_user_secret_env().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims = infrastructure::jwt::verify_token(secret_env.to_string(),
     token).map_err(|_| StatusCode::UNAUTHORIZED)?;

     let brawler_id = claims
     .sub
     .parse::<i32>()
     .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(brawler_id);
    // pass to next middleware and return Response<Body> later
    Ok(next.run(req).await)
}