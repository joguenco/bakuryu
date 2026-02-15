use crate::DBPool;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder, get};
use diesel::sql_types::Text;
use diesel::{QueryableByName, RunQueryDsl, sql_query};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct VersionResponse {
    version: String,
    version_database: String,
}

#[derive(QueryableByName)]
struct Version {
    #[diesel(sql_type = Text)]
    version_database: String,
}

#[get("/version")]
pub async fn version(db: Data<DBPool>) -> impl Responder {
    match db.get() {
        Ok(mut conn) => {
            let version_db = sql_query("SELECT version() as version_database")
                .load::<Version>(&mut conn)
                .map(|v| {
                    v.first()
                        .map(|x| x.version_database.clone())
                        .unwrap_or_else(|| "Unknown".to_string())
                })
                .unwrap_or_else(|_| "Error".to_string());

            let p = VersionResponse {
                version: "1.0.0".to_string(),
                version_database: version_db,
            };
            HttpResponse::Ok().json(p)
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
