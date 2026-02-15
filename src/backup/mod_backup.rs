use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_web::{Error, HttpResponse, Result, post};
use std::fs;

#[derive(Debug, MultipartForm)]
pub struct FormWithFile {
    pub name: Text<String>,
    pub file_data: TempFile,
}

#[post("/backup")]
pub async fn backup(
    MultipartForm(form): MultipartForm<FormWithFile>,
) -> Result<HttpResponse, Error> {
    print!("Name: {:?}\n", form.name.as_str());

    let file_name = form
        .file_data
        .file_name
        .unwrap_or_else(|| "unknown".to_string());
    println!("File name: {:?}", file_name);
    let file_path = format!("/home/jorgeluis/tmp/{}", file_name);
    println!("File path: {:?}", file_path);

    fs::copy(&form.file_data.file.path(), &file_path).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to save file: {}", e))
    })?;

    Ok(HttpResponse::Ok().body("Data process is successful"))
}
