use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passport {
    pub refresh_token: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub display_name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}