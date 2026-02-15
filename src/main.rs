mod auth;
mod backup;
mod db;
mod models;
mod ping;
mod schema;
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

use actix_web::{App, HttpServer, middleware::Logger, web};
use actix_web_httpauth::middleware::HttpAuthentication;

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
