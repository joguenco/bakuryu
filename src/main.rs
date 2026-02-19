mod auth;
mod backup;
mod common;
mod db;
mod ping;
mod version;
use crate::auth::auth_validator;
use crate::backup::mod_backup::backup as backup_handler;
use crate::db::connect;
use crate::ping::mod_ping::ping as ping_handler;
use crate::version::mod_version::version as version_handler;
use diesel::{
    PgConnection,
    r2d2::{self, ConnectionManager},
};
use dotenvy::dotenv;
use std::env;
use std::time::Duration;

use actix_multipart::form::MultipartFormConfig;
use actix_web::{App, HttpServer, middleware::Logger, web};
use actix_web_httpauth::middleware::HttpAuthentication;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").expect("failed to get PORT");
    let con = connect();
    let pool_data = web::Data::new(con);
    common::init_logging();

    HttpServer::new(move || {
        let auth = HttpAuthentication::with_fn(auth_validator);
        App::new()
            .app_data(pool_data.clone())
            .app_data(web::PayloadConfig::new(2048 * 1024 * 1024))
            .service(ping_handler)
            .service(
                web::scope("")
                    .wrap(auth)
                    .service(version_handler)
                    .service(backup_handler),
            )
            .app_data(MultipartFormConfig::default().total_limit(2048 * 1024 * 1024))
            .wrap(Logger::default())
    })
    .client_request_timeout(Duration::from_secs(180))
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .workers(3)
    .run()
    .await
}
