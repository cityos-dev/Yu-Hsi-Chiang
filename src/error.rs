use actix_web::{
    error::ResponseError,
    http::StatusCode,
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum FileHandlingError {
    RequiredFieldNotFound,
    UnsupportedMediaType,
    FileExists,
    FileNotFound,
    DatabaseError(crate::file_metadata::DatabaseError),
}

impl ResponseError for FileHandlingError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            FileHandlingError::RequiredFieldNotFound => StatusCode::BAD_REQUEST,
            FileHandlingError::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            FileHandlingError::FileExists => StatusCode::CONFLICT,
            FileHandlingError::FileNotFound => StatusCode::NOT_FOUND,
            FileHandlingError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<crate::file_metadata::DatabaseError> for FileHandlingError {
    fn from(error: crate::file_metadata::DatabaseError) -> Self {
        Self::DatabaseError(error)
    }
}
