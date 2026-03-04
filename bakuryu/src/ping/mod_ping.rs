use actix_web::{Responder, Result, get, web};
use serde::Serialize;

#[derive(Serialize)]
struct PingResponse {
    message: String,
}

#[get("/ping")]
pub async fn ping() -> Result<impl Responder> {
    let p = PingResponse {
        message: "pong".to_string(),
    };
    Ok(web::Json(p))
}
