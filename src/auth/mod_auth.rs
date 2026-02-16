use crate::DBPool;
use crate::models::AccessToken;
use actix_web::{Error, dev::ServiceRequest, error};
use actix_web::{HttpMessage, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use diesel::prelude::*;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::env;
use std::ops::DerefMut;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    iat: f64,
    exp: f64,
    aud: String,
    sub: String,
    client: String,
    pub(crate) name: String,
    email: String,
    role: Vec<String>,
    service: String,
}

pub async fn auth_validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(credentials) = credentials else {
        return Err((error::ErrorBadRequest("Unauthorized"), req));
    };

    let bearer_token = credentials.token().to_string();

    let pool = req.app_data::<web::Data<DBPool>>().unwrap();

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err((error::ErrorInternalServerError(e), req)),
    };

    use crate::schema::access_tokens::dsl::{access_tokens, status, token};

    let token_exists = access_tokens
        .filter(token.eq(&bearer_token))
        .filter(status)
        .select(AccessToken::as_select())
        .first::<AccessToken>(conn.deref_mut())
        .optional();

    match token_exists {
        Ok(Some(found_token)) => {
            req.extensions_mut().insert(found_token.token.clone());
            Ok(req)
        }
        Ok(None) => Err((error::ErrorUnauthorized("Unauthorized"), req)),
        Err(e) => Err((error::ErrorInternalServerError(e), req)),
    }
}

pub fn get_claims_from_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret_env = env::var("PRIVATE_KEY").expect("failed to get PRIVATE_KEY from env");
    let secret = secret_env.as_bytes();
    let decoding_key = DecodingKey::from_secret(secret);
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    validation.set_audience(&["resolvedor.dev"]);
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}
