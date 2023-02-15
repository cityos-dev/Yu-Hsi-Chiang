mod error;
mod uploaded_files;

use crate::error::FileHandlingError;
use crate::uploaded_files::UploadedFiles;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{
    delete, get, http::header, post, web, App, HttpResponse, HttpServer, Responder, Result,
};
use futures::future::ready;
use futures_util::{StreamExt as _, TryStreamExt as _};
use mongodb::Client;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::env;

const SUPPORTED_MIME_TYPES: [(mime::Name<'_>, mime::Name<'_>); 2] =
    [(mime::VIDEO, mime::MP4), (mime::VIDEO, mime::MPEG)];

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body(include_str!("index.html"))
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/files/{fileid}")]
async fn get_file(db: web::Data<UploadedFiles>, fileid: web::Path<String>) -> Result<NamedFile> {
    if let Some(metadata) = db.get(&fileid).await? {
        let file = NamedFile::open_async(format!("files/{}", metadata.fileid)).await?;
        return Ok(file
            .set_content_type(metadata.mime)
            .set_content_disposition(header::ContentDisposition {
                disposition: header::DispositionType::Inline,
                parameters: vec![header::DispositionParam::Filename(metadata.name)],
            }));
    }
    Err(FileHandlingError::FileNotFound.into())
}

#[delete("/files/{fileid}")]
async fn delete_file(
    db: web::Data<UploadedFiles>,
    fileid: web::Path<String>,
) -> Result<HttpResponse> {
    if db.delete(&fileid).await? {
        web::block(move || fs::remove_file(format!("files/{}", fileid))).await??;
        return Ok(HttpResponse::NoContent().into());
    }
    Err(FileHandlingError::FileNotFound.into())
}

#[get("/files")]
async fn list_files(db: web::Data<UploadedFiles>) -> Result<HttpResponse> {
    let files = db.list().await?;
    Ok(HttpResponse::Ok().json(files).into())
}

#[post("/files")]
async fn upload_file(db: web::Data<UploadedFiles>, mut payload: Multipart) -> Result<HttpResponse> {
    while let Some(mut field) = payload.try_next().await? {
        if field.name() != "data" {
            field.for_each(|_| ready(())).await;
            continue;
        }
        let filename = field
            .content_disposition()
            .get_filename()
            .ok_or(FileHandlingError::RequiredFieldNotFound)?
            .to_string();
        let content_type = field
            .content_type()
            .ok_or(FileHandlingError::RequiredFieldNotFound)?;
        if !SUPPORTED_MIME_TYPES.contains(&(content_type.type_(), content_type.subtype())) {
            field.for_each(|_| ready(())).await;
            return Err(FileHandlingError::UnsupportedMediaType.into());
        }
        let fileid = db
            .create(&filename, content_type)
            .await?
            .ok_or(FileHandlingError::FileExists)?;
        let write_file_name = format!("files/{}", &fileid);
        let mut size = 0;
        let mut fd = web::block(move || File::create(write_file_name)).await??;
        while let Some(chunk) = field.try_next().await? {
            size += chunk.len();
            fd = web::block(move || fd.write_all(&chunk).map(|_| fd)).await??;
        }
        web::block(move || fd.sync_all()).await??;
        db.update_size(&fileid, size).await?;
        return Ok(HttpResponse::Created()
            .append_header((header::LOCATION, format!("/v1/files/{}", &fileid)))
            .finish()
            .into());
    }
    Err(FileHandlingError::RequiredFieldNotFound.into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Client::with_uri_str(env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into()))
        .await
        .expect("failed to connect")
        .database("video-storage");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(UploadedFiles::new(db.clone())))
            .service(
                web::scope("v1")
                    .service(health)
                    .service(list_files)
                    .service(upload_file)
                    .service(get_file)
                    .service(delete_file), // .service(upload::get_files),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
