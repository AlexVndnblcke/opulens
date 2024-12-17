use axum::extract::multipart::Field;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct Upload {
    pub user_id: Uuid,
    file: UploadFile,
    source: String,
    timestamp: DateTime<Utc>,
}

impl Upload {
    pub fn new(user_id: Uuid, file: UploadFile, source: String, timestamp: Option<DateTime<Utc>>) -> Self{
        let timestamp = match timestamp {
            Some(_timestamp) => _timestamp,
            None => Utc::now(),
        };
        Self {
            user_id,
            file,
            source,
            timestamp,
        }
    }

    pub fn get_file_name(&self) -> &str {
        &self.file.file_name
    }
}


#[derive(Debug)]
pub enum FileError {
    MultipartError,
    MissingName,
    MissingContentType,

}

#[derive(Debug, Serialize)]
pub struct UploadFile {
    content: Bytes,
    pub file_name: String,
    content_type: String,
}

impl UploadFile {
    pub async fn from_field(field: Field<'_>) -> Result<Self, FileError> {
        let file_name = field.file_name().ok_or_else(|| FileError::MissingName)?.to_string();
        let content_type = field.content_type().ok_or_else(|| FileError::MissingContentType)?.to_string();
        let content = field.bytes().await.map_err(|_e| FileError::MultipartError)?;

        Ok(Self {
            content,
            file_name,
            content_type,
        })
    }
}
