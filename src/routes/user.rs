use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::{debug, info, warn};

use tokio::sync::RwLock;

use crate::{
    service::{DataAccessService, Identify},
    uuid::Uuid,
    Context,
};

use crate::models::user::User;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NewUser {
    username: String,
    email_address: String,
    // TODO: add hashed and salted password, bcrypt module?
}

#[debug_handler]
pub async fn create_user(
    State(state): State<Arc<RwLock<Context>>>,
    Json(payload): Json<NewUser>,
) -> Response {
    let new_id = Uuid::new_v4();
    let user = User::new(new_id, payload.username, payload.email_address);

    let mut state_write_lock = state.write().await;
    if let Err(message) = state_write_lock.data_service.create(&user).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, message).into_response();
    }
    info!("new user {:?} is added to the user set", user);

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user)).into_response()
}

#[debug_handler]
pub async fn update_user(
    State(state): State<Arc<RwLock<Context>>>,
    Json(user): Json<User>,
) -> Response {
    let mut state_write_lock = state.write().await;

    if let Err(message) = state_write_lock.data_service.update(&user).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, message).into_response();
    }
    info!("user {:?} is updated in the user set", &user.id());

    // this will be converted into a JSON response
    // with a status code of `204 No content`
    StatusCode::NO_CONTENT.into_response()
}

#[debug_handler]
pub async fn get_user(
    State(state): State<Arc<RwLock<Context>>>,
    Path(id): Path<String>,
) -> Response {
    let state_write_lock = state.write().await;

    let id: Uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(e) => {
            warn!(
                "attempt to get user with invalid id format, error: {:?}",
                e.to_string()
            );
            return (StatusCode::BAD_REQUEST, "Invalid user id format").into_response();
        }
    };

    let user = match state_write_lock.data_service.read(&id).await {
        Err(message) => return (StatusCode::INTERNAL_SERVER_ERROR, message).into_response(),
        Ok(None) => {
            debug!("attempt to retrieve non-existing user {:?}", &id);
            return (
                StatusCode::NOT_FOUND,
                format!("No user found for id {:?}", &id),
            )
                .into_response();
        }
        Ok(Some(user)) => user,
    };
    info!("user {:?} is updated in the user set", &id);

    (StatusCode::OK, Json::<User>::from(user)).into_response()
}
