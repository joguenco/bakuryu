use crate::DBPool;
use crate::auth::mod_auth::get_claims_from_token;
use crate::common::ErrorMessage;
use crate::db::models::Entity;
use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, post, web};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use log;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, MultipartForm)]
struct FormWithFile {
    sha2_code: Text<String>,
    #[multipart(limit = "2048MB")]
    file_data: TempFile,
}

#[derive(Serialize)]
struct BackupResponse {
    message: String,
}

#[post("/backup")]
pub async fn backup(
    req: HttpRequest,
    MultipartForm(form): MultipartForm<FormWithFile>,
) -> Result<impl Responder, ErrorMessage> {
    let token_from_auth = req.extensions().get::<String>().cloned();

    let value = token_from_auth
        .clone()
        .unwrap_or_else(|| "No token found".to_string());

    let claim = match get_claims_from_token(&value) {
        Ok(claims) => claims,
        Err(e) => {
            println!("Error extracting claims: {:?}", e);
            return Err(ErrorMessage {
                code: 401,
                message: "Invalid token".to_string(),
            });
        }
    };

    let pool = req.app_data::<web::Data<DBPool>>().unwrap();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return Err(ErrorMessage {
                code: 500,
                message: "Error to get connection to database".to_string(),
            });
        }
    };

    let file_name = form
        .file_data
        .file_name
        .unwrap_or_else(|| "unknown".to_string());

    let folder_path_to_save = get_path(&claim.name, &mut conn);

    if folder_path_to_save.is_empty() {
        return Err(ErrorMessage {
            code: 404,
            message: "Folder not found".to_string(),
        });
    }

    let file_path = format!("{}/{}", folder_path_to_save, file_name);

    let _ = fs::copy(&form.file_data.file.path(), &file_path).map_err(|e| ErrorMessage {
        code: 500,
        message: format!("Error to save file: {}", e),
    });

    let p = BackupResponse {
        message: "Upload successful".to_string(),
    };
    Ok(web::Json(p))
}

fn get_path(name_from_jwt: &str, conn: &mut PgConnection) -> String {
    use crate::db::schema::entity::dsl::{entity, name};

    let result_entity = entity
        .filter(name.eq(&name_from_jwt))
        .select(Entity::as_select())
        .first::<Entity>(conn)
        .optional();

    match result_entity {
        Ok(Some(found_entity)) => {
            let is_created = create_folder_if_not_exist(&found_entity.folder_path);
            if is_created {
                log::info!(
                    "get_path - folder for {}: {}",
                    name_from_jwt,
                    found_entity.folder_path
                );
                found_entity.folder_path
            } else {
                log::error!(
                    "get_path - folder not found for {}: {}",
                    name_from_jwt,
                    found_entity.folder_path
                );
                "".to_string()
            }
        }
        Ok(None) => {
            log::warn!("get_path - no entity found for name: {}", name_from_jwt);
            "".to_string()
        }
        Err(e) => {
            log::error!("get_path - database error for {}: {:?}", name_from_jwt, e);
            "".to_string()
        }
    }
}

fn create_folder_if_not_exist<P: AsRef<Path>>(path: P) -> bool {
    fs::create_dir_all(path).is_ok()
}
