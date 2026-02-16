use crate::DBPool;
use crate::auth::mod_auth::get_claims_from_token;
use crate::models::Entity;
use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, Result, post, web};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::fs;
use std::path::Path;

#[derive(Debug, MultipartForm)]
struct FormWithFile {
    sha2_code: Text<String>,
    file_data: TempFile,
}

#[post("/backup")]
pub async fn backup(
    req: HttpRequest,
    MultipartForm(form): MultipartForm<FormWithFile>,
) -> Result<HttpResponse, Error> {
    let token_from_auth = req.extensions().get::<String>().cloned();

    let value = token_from_auth
        .clone()
        .unwrap_or_else(|| "No token found".to_string());

    let claim = match get_claims_from_token(&value) {
        Ok(claims) => claims,
        Err(e) => {
            println!("Error extracting claims: {:?}", e);
            return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
        }
    };

    let pool = req.app_data::<web::Data<DBPool>>().unwrap();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => return Err(actix_web::error::ErrorInternalServerError(e)),
    };

    let file_name = form
        .file_data
        .file_name
        .unwrap_or_else(|| "unknown".to_string());

    let file_path = format!("{}/{}", get_path(&claim.name, &mut conn), file_name);

    fs::copy(&form.file_data.file.path(), &file_path).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to save file: {}", e))
    })?;

    Ok(HttpResponse::Ok().body("Data process is successful"))
}

fn get_path(name_from_jwt: &str, conn: &mut PgConnection) -> String {
    use crate::schema::entity::dsl::{entity, name};

    let result_entity = entity
        .filter(name.eq(&name_from_jwt))
        .select(Entity::as_select())
        .first::<Entity>(conn)
        .optional();

    match result_entity {
        Ok(Some(found_entity)) => {
            let _ = create_folder_if_not_exist(&found_entity.folder_path);
            found_entity.folder_path
        }
        Ok(None) => {
            println!("No entity found for name: {}", name_from_jwt);
            "/tmp".to_string()
        }
        Err(e) => {
            println!("Database error: {:?}", e);
            "/tmp".to_string()
        }
    }
}

fn create_folder_if_not_exist<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}
