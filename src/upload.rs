use actix_multipart::Multipart;
use actix_web::{
    error,
    http::{header, StatusCode},
    post, web, Error, HttpResponse, Result,
};
use derive_more::{Display, Error};
use futures_util::StreamExt as _;
use mime;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

const SUPPORTED_MIME_TYPES: [(mime::Name<'_>, mime::Name<'_>); 2] =
    [(mime::VIDEO, mime::MP4), (mime::VIDEO, mime::MPEG)];

#[derive(Debug, Display, Error)]
enum UploadHandlingErrors {
    #[display(fmt = "field not found")]
    FieldNotFound,

    #[display(fmt = "content type not found")]
    ContentTypeNotFound,

    #[display(fmt = "unsupported media type")]
    UnsupportedMediaType,
}

impl error::ResponseError for UploadHandlingErrors {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UploadHandlingErrors::FieldNotFound | UploadHandlingErrors::ContentTypeNotFound => {
                StatusCode::BAD_REQUEST
            }
            UploadHandlingErrors::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        }
    }
}

#[post("/files")]
async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    if let Some(item) = payload.next().await {
        let mut field = item?;
        let filename = field
            .content_disposition()
            .get_filename()
            .ok_or(UploadHandlingErrors::FieldNotFound)?
            .to_string();
        let content_type = field
            .content_type()
            .ok_or(UploadHandlingErrors::FieldNotFound)?;
        if !SUPPORTED_MIME_TYPES.contains(&(content_type.type_(), content_type.subtype())) {
            return Err(UploadHandlingErrors::UnsupportedMediaType.into());
        }
        let mut file = web::block(move || File::create("files/1")).await??;
        while let Some(chunk) = field.next().await {
            file = web::block(move || file.write_all(&chunk.unwrap()).map(|_| file)).await??;
        }
        web::block(move || file.sync_all()).await??;
    }
    Ok(HttpResponse::Created()
        .append_header((header::LOCATION, "/v1/files/1"))
        .finish()
        .into())
}
