pub mod jwt_model;
pub mod authentication_model;

use anyhow::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use self::jwt_model::Claims;

pub fn generate_token(secret: String, claims: &Claims) -> Result<String> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}

pub fn verify_token(secret: String, token: String) -> Result<Claims> {
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}