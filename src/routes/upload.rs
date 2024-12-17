use std::sync::Arc;

use axum::extract::multipart::Multipart;
use axum::extract::State;
use axum::http::StatusCode;
use chrono::Utc;
use tokio::sync::RwLock;
use tracing::{event, Level};

use crate::models::context::Context;
use crate::models::upload::{Upload, UploadFile};
use crate::uuid::Uuid;


pub async fn upload(
    State(state): State<Arc<RwLock<Context>>>,
    mut multipart: Multipart,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut file: Option<UploadFile> = None;
    let mut user_id: Option<String> = None;
    let mut source: Option<String> = None;

    while let next_field_result = multipart.next_field().await {
        let field = match next_field_result {
            Ok(Some(f)) => f,
            Ok(None) => {
                event!(Level::WARN, "upload done");
                break;
            },
            Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
        };

        match field.name() {
            // Define upload fields.
            Some("source") => source = Some(field.text().await.or_else(|e| Err((StatusCode::BAD_REQUEST, e.to_string())))?),
            Some("user_id") => user_id = Some(field.text().await.or_else(|e| Err((StatusCode::BAD_REQUEST, e.to_string())))?),
            Some("file") => {
                event!(Level::DEBUG, "attempt to parse multipart field file to UploadFile");

                let transform_result = UploadFile::from_field(field).await;
                if let Ok(uploaded_file) = transform_result {
                    event!(Level::INFO, "multipart field was successfully parsed as file");
                    file = Some(uploaded_file);
                } else {
                    event!(Level::WARN, "failed attempt to parse field as file");
                    return Err((StatusCode::BAD_REQUEST, format!("File error: {:?}", transform_result.unwrap_err())));
                }

            },
            Some(_) | None => continue,
        };
    }

    event!(Level::DEBUG, "all multiparts have been processed");

    let user_id = user_id.ok_or((StatusCode::BAD_REQUEST, "no user id given in the multipart request".to_string()))?;
    let user_id = Uuid::parse_str(&user_id).map_err(|_e| (StatusCode::BAD_REQUEST, "Invalid user id format, must be UUID".into()))?;
    let file = file.ok_or_else(|| (StatusCode::BAD_REQUEST, "No file was passed".into()))?;
    let source = source
            .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing `source` content".into()))?;

    let upload = Upload::new(user_id, file, source, Some(Utc::now()));

    event!(Level::INFO, "new upload for {:?} with name {name}", &upload.user_id, name=upload.get_file_name());

    let mut state_lock = state.write().await;
    state_lock.add_upload(upload).await;

    Ok(StatusCode::CREATED)
}
