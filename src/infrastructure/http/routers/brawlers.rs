use std::sync::Arc;

use axum::{ Extension, Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}};

use crate::{
    application::use_cases::{brawlers::BrawlersUseCase, authentication::AuthenticationUseCase},
    domain::{
        repositories::brawlers::BrawlerRepository,
        value_objects::{brawler_model::RegisterBrawlerModel, uploaded_image::UploadedAvartar},
    },
    infrastructure::{database::{
        postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres,
    }, http::middleware::auth::authorization, jwt::authentication_model::LoginModel},
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawlers_repository = Arc::new(BrawlerPostgres::new(db_pool));
    let brawlers_use_case = Arc::new(BrawlersUseCase::new(Arc::clone(&brawlers_repository)));
    let authentication_use_case = Arc::new(AuthenticationUseCase::new(Arc::clone(&brawlers_repository)));

    let protected_router = Router::new()
        .route("/avatar", post(upload_avatar))
        .route("/my-missions", get(get_missions))
        .route_layer(axum::middleware::from_fn(authorization))
        .with_state(brawlers_use_case.clone());

    Router::new()
        .route("/register", post(register))
        .with_state((brawlers_use_case, authentication_use_case))
        .merge(protected_router)
}

pub async fn register<T>(
    State((brawlers_use_case, authentication_use_case)): State<(Arc<BrawlersUseCase<T>>, Arc<AuthenticationUseCase<T>>)>,
    Json(register_brawler_model): Json<RegisterBrawlerModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    let username = register_brawler_model.username.clone();
    let password = register_brawler_model.password.clone();

    match brawlers_use_case.register(register_brawler_model).await {
        Ok(_) => {
            let login_model = LoginModel { username, password };
            match authentication_use_case.login(login_model).await {
                Ok(passport) => (
                    StatusCode::CREATED, 
                    Json(serde_json::json!({
                        "access_token": passport.access_token,
                        "token_type": passport.token_type,
                        "expires_in": passport.expires_in,
                        "display_name": passport.display_name,
                        "avatar_url": passport.avatar_url,
                        "message": "Register and Login successfully"
                    }))
                ).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"message": e.to_string()}))).into_response(),
            }
        },
        Err(e) => {
            let error_message = e.to_string();
            let status = if error_message.contains("Username already exists") {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, Json(serde_json::json!({ "message": error_message }))).into_response()
        },
    }
}


pub async fn upload_avatar<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
    Json(upload_image): Json<UploadedAvartar>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
  {
    match brawlers_use_case
        .upload_avatar(upload_image.base64_string, brawler_id)
        .await
    {
        Ok(uploaded_image) => (StatusCode::CREATED, Json(serde_json::json!({ "url": uploaded_image }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_missions<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.get_my_missions(brawler_id).await {
        Ok(missions) => (StatusCode::OK, Json(missions)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json::<serde_json::Value>(serde_json::json!({"message": e.to_string()}))).into_response(),
    }
}
