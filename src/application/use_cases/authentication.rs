
use std::{ sync::Arc};
use anyhow::Result;
use chrono::{Duration, Utc};

use crate::{domain::repositories::brawlers::BrawlerRepository, infrastructure::{self, jwt::{authentication_model::LoginModel, jwt_model::{Claims, Passport}}}};

pub struct AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}




impl <T> AuthenticationUseCase<T> 
    where
        T: BrawlerRepository + Send + Sync,
    {
        pub fn new(brawler_repository: Arc<T>) -> Self {
            Self { brawler_repository }

        }

        pub async fn login(&self, login_model: LoginModel) -> Result<Passport>{

            let secret_env = get_user_secret_env()?;
            let username = login_model.username.clone();

            let brawler = self.brawler_repository.find_by_username(username).await?;

            let hash_password = brawler.password;
            let login_password = login_model.password;

            if !infrastructure::argon2::verify(login_password, hash_password)? {
                return Err(anyhow::anyhow!("Invalid username or password"));
            }

            let access_token_claims = Claims {
                sub: brawler.id.to_string(),
                exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
                iat: Utc::now().timestamp() as usize,
            };

            let access_token = 
            infrastructure::jwt::generate_token(secret_env.clone(), &access_token_claims)?;

            let refresh_token_claims = Claims {
                sub: brawler.id.to_string(),
                exp: (Utc::now() + Duration::days(3)).timestamp() as usize,
                iat: Utc::now().timestamp() as usize,
            };

            let refresh_token = 
            infrastructure::jwt::generate_token(secret_env, &refresh_token_claims)?;
            
            Ok(Passport{
                refresh_token,
                access_token,
                token_type: "Bearer".to_string(),
                expires_in: access_token_claims.exp,
                display_name: brawler.username, // Using username as display_name for now
                avatar_url: None, // Avatar not yet supported in DB
            })


        }

        pub async fn refresh_token(){
            
        }
    }
    
pub fn get_user_secret_env() -> Result<String>{
    let secret_env = std::env::var("JWT_USER_SECRET")
    .map_err(|_| anyhow::anyhow!("JWT_USER_SECRET environment variable not set"))?;
    Ok(secret_env)
}