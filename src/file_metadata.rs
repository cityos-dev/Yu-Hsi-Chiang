use actix_web::{
    error::ResponseError,
    http::StatusCode,
    HttpResponse,
};
use bson::oid::ObjectId;
use derive_more::{Display, Error};
use futures_util::TryStreamExt as _;
use mime::Mime;
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};
use std::str::FromStr as _;

#[derive(Debug, Display, Error)]
pub enum DatabaseError {
    MongoError(mongodb::error::Error),
    ObjectIdError,
}

type Result<T> = std::result::Result<T, DatabaseError>;

#[derive(Clone)]
pub struct FileMetadatas {
    db: Database,
}

impl FileMetadatas {
    pub fn new(db: Database) -> Self {
        FileMetadatas { db }
    }

    pub async fn create(self: &Self, name: &String, mime: &Mime) -> Result<Option<String>> {
        let collection = self.db.collection("videos");
        if collection
            .find_one(doc! { "name": name }, None)
            .await?
            .is_some()
        {
            return Ok(None);
        }
        collection
            .insert_one(
                doc! {
                    "name": name,
                    "mime": {
                        "type_": mime.type_().to_string(),
                        "subtype": mime.subtype().to_string(),
                    },
                    "size": 0,
                },
                /* options= */ None,
            )
            .await?
            .inserted_id
            .as_object_id()
            .ok_or(DatabaseError::ObjectIdError)
            .map(|object_id| Some(object_id.to_hex()))
    }

    pub async fn update_size(self: &Self, id: &String, size: usize) -> Result<bool> {
        let collection: Collection<FileMetadataInternal> = self.db.collection("videos");
        Ok(match ObjectId::parse_str(id).ok() {
            Some(object_id) => {
                collection
                    .find_one_and_update(
                        doc! { "_id": object_id },
                        doc! { "$set": { "size": size as i64 }},
                        /* options= */ None,
                    )
                    .await
                    .map(|_| true)?
            }
            None => false,
        })
    }

    pub async fn get(self: &Self, id: &String) -> Result<Option<FileMetadata>> {
        let collection: Collection<FileMetadataInternal> = self.db.collection("videos");
        Ok(match ObjectId::parse_str(id).ok() {
            Some(object_id) => {
                collection
                    .find_one(doc! { "_id": object_id }, /* options= */ None)
                    .await?
                    .map(|f| f.into())
            }
            None => None,
        })
    }

    pub async fn list(self: &Self) -> Result<Vec<FileMetadata>> {
        let collection: Collection<FileMetadataInternal> = self.db.collection("videos");
        let cursor = collection.find(None, None).await?;
        Ok(cursor.map_ok(|f| f.into()).try_collect().await?)
    }

    pub async fn delete(self: &Self, id: &String) -> Result<bool> {
        let collection: Collection<FileMetadataInternal> = self.db.collection("videos");
        Ok(match ObjectId::parse_str(id).ok() {
            Some(object_id) => {
                collection
                    .find_one_and_delete(doc! { "_id": object_id }, /* options= */ None)
                    .await?
                    .is_some()
            }
            None => false,
        })
    }
}

impl From<mongodb::error::Error> for DatabaseError {
    fn from(error: mongodb::error::Error) -> Self {
        Self::MongoError(error)
    }
}

impl From<bson::oid::Error> for DatabaseError {
    fn from(_: bson::oid::Error) -> Self {
        Self::ObjectIdError
    }
}

impl ResponseError for DatabaseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct FileMetadataInternal {
    #[serde(rename = "_id")]
    id: bson::oid::ObjectId,
    mime: MimeInternal,
    name: String,
    size: i64,
}

#[derive(Serialize, Clone, Debug)]
pub struct FileMetadata {
    pub fileid: String,
    pub name: String,
    pub size: i64,
    pub created_at: String,
    #[serde(skip_serializing)]
    pub mime: mime::Mime,
}

impl From<FileMetadataInternal> for FileMetadata{
    fn from(file: FileMetadataInternal) -> Self {
        FileMetadata{
            fileid: file.id.to_hex(),
            name: file.name,
            size: file.size,
            created_at: file.id.timestamp().try_to_rfc3339_string().unwrap(),
            mime: file.mime.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct MimeInternal {
    type_: String,
    subtype: String,
}

impl From<&Mime> for MimeInternal {
    fn from(mime: &Mime) -> Self {
        MimeInternal {
            type_: mime.type_().to_string(),
            subtype: mime.subtype().to_string(),
        }
    }
}

impl Into<Mime> for MimeInternal {
    fn into(self: Self) -> Mime {
        Mime::from_str(format!("{}/{}", self.type_, self.subtype).as_str()).unwrap()
    }
}
