use crate::DBPool;
use crate::models::AccessToken;
use actix_web::web;
use actix_web::{Error, dev::ServiceRequest, error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use diesel::prelude::*;
use std::ops::DerefMut;

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
        Ok(Some(_)) => Ok(req),
        Ok(None) => Err((error::ErrorUnauthorized("Unauthorized"), req)),
        Err(e) => Err((error::ErrorInternalServerError(e), req)),
    }
}
