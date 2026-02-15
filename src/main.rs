mod backup;
mod models;
mod ping;
mod schema;
mod version;
use crate::backup::mod_backup::backup as backup_handler;
use crate::ping::mod_ping::ping as ping_handler;
use crate::version::mod_version::version as version_handler;
use diesel::{
    PgConnection,
    r2d2::{self, ConnectionManager},
};
use dotenvy::dotenv;
use std::env;

use crate::models::AccessToken;
use actix_web::{App, Error, HttpServer, dev::ServiceRequest, error, middleware::Logger, web};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use diesel::prelude::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let con = connect();
    let pool_data = web::Data::new(con);

    HttpServer::new(move || {
        let auth = HttpAuthentication::with_fn(auth_validator);
        App::new()
            .app_data(pool_data.clone())
            .wrap(Logger::default())
            .service(ping_handler)
            .service(
                web::scope("")
                    .wrap(auth)
                    .service(version_handler)
                    .service(backup_handler),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .workers(3)
    .run()
    .await
}

pub fn connect() -> DBPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("faild to get DB Url");
    let db = ConnectionManager::new(db_url);

    r2d2::Pool::builder()
        .max_size(5)
        .build(db)
        .expect("faild to create db pool")
}

async fn auth_validator(
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
        .first::<AccessToken>(&mut conn)
        .optional();

    match token_exists {
        Ok(Some(_)) => Ok(req),
        Ok(None) => Err((error::ErrorUnauthorized("Unauthorized"), req)),
        Err(e) => Err((error::ErrorInternalServerError(e), req)),
    }
}
