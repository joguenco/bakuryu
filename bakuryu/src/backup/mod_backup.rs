use crate::DBPool;
use crate::auth::mod_auth::get_claims_from_token;
use crate::common::ErrorMessage;
use crate::db::models::Entity;
use crate::db::models::FileDetail;
use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_web::{HttpMessage, HttpRequest, Responder, post, web};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use log;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, MultipartForm)]
struct FormWithFile {
    sha2: Text<String>,
    #[multipart(limit = "2048MB")]
    file: TempFile,
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

    let file_name = form.file.file_name.unwrap_or_else(|| "unknown".to_string());

    let entity_result = get_path(&claim.name, &mut conn);

    if entity_result.folder_path.is_empty() {
        return Err(ErrorMessage {
            code: 404,
            message: "Folder not found".to_string(),
        });
    }

    let file_path = format!("{}/{}", entity_result.folder_path, file_name);

    fs::copy(&form.file.file.path(), &file_path).map_err(|e| ErrorMessage {
        code: 500,
        message: format!("Error to save file: {}", e),
    })?;

    // Compute and validate SHA-256 of the saved file
    let sha256 = get_sha256_of_file(&file_path, &form.sha2.0)?;

    save_info_in_db(&file_name, &sha256, entity_result.id, &mut conn)?;

    let p = BackupResponse {
        message: "Upload successful".to_string(),
    };
    Ok(web::Json(p))
}

fn save_info_in_db(
    file_name: &str,
    sha256: &str,
    entity_id: i32,
    conn: &mut PgConnection,
) -> Result<(), ErrorMessage> {
    use crate::db::schema::file_details::dsl::file_details;

    let new_file = FileDetail {
        size: 0,
        sha256: sha256.to_string(),
        is_sha256_valid: Some(true),
        is_restored: None,
        file_name: file_name.to_string(),
        entity_id,
    };

    diesel::insert_into(file_details)
        .values(&new_file)
        .execute(conn)
        .map_err(|e| ErrorMessage {
            code: 500,
            message: format!("Error inserting file info into database: {}", e),
        })?;

    Ok(())
}

fn get_sha256_of_file(file_path: &str, input_sha2: &str) -> Result<String, ErrorMessage> {
    // Compute SHA-256 of the saved file
    let mut file = fs::File::open(file_path).map_err(|e| ErrorMessage {
        code: 500,
        message: format!("Error to open saved file for hashing: {}", e),
    })?;

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer).map_err(|e| ErrorMessage {
            code: 500,
            message: format!("Error to read saved file for hashing: {}", e),
        })?;

        if n == 0 {
            break;
        }

        hasher.update(&buffer[..n]);
    }

    let computed_hash = hasher.finalize();
    let computed_sha_hex = format!("{:x}", computed_hash);

    let expected_sha = input_sha2.trim().to_lowercase();

    log::info!(
        "backup - provided sha256: {}, computed sha256: {}",
        expected_sha,
        computed_sha_hex
    );

    if computed_sha_hex != expected_sha {
        return Err(ErrorMessage {
            code: 400,
            message: "SHA-256 mismatch".to_string(),
        });
    }

    Ok(computed_sha_hex)
}

fn get_path(name_from_jwt: &str, conn: &mut PgConnection) -> Entity {
    use crate::db::schema::entities::dsl::{entities, name};

    let result_entity: Result<Option<Entity>, diesel::result::Error> = entities
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
                found_entity
            } else {
                log::error!(
                    "get_path - folder not found for {}: {}",
                    name_from_jwt,
                    found_entity.folder_path
                );
                found_entity
            }
        }
        Ok(None) => {
            log::warn!("get_path - no entity found for name: {}", name_from_jwt);
            Entity {
                id: 0,
                name: "".to_string(),
                folder_path: "".to_string(),
                store_type: None,
                observation: None,
            }
        }
        Err(e) => {
            log::error!("get_path - database error for {}: {:?}", name_from_jwt, e);
            Entity {
                id: 0,
                name: "".to_string(),
                folder_path: "".to_string(),
                store_type: None,
                observation: None,
            }
        }
    }
}

fn create_folder_if_not_exist<P: AsRef<Path>>(path: P) -> bool {
    fs::create_dir_all(path).is_ok()
}
