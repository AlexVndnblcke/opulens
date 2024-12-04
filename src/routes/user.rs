use std::sync::Arc;

use tracing::{event, instrument, Level};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    debug_handler,
};

use tokio::sync::RwLock;

use crate::{uuid::Uuid, Context};

use serde::Deserialize;
use crate::models::user::User;


#[derive(Deserialize, Debug)]
pub struct NewUser {
    username: String,
    email_address: String,
    // TODO: add hashed and salted password, bcrypt module?
}


#[debug_handler]
#[instrument]
pub async fn create_user(
    State(state): State<Arc<RwLock<Context>>>,
    Json(payload): Json<NewUser>,
) -> (StatusCode, Json<User>) {

    let new_id = Uuid::new_v4();
    // insert your application logic here
    let user = User::new(
        new_id,
        payload.username,
        payload.email_address,
    );

    let mut state_write_lock = state.write().await;
    state_write_lock.add_user(user.clone()).await;
    event!(Level::INFO, "new user {:?} is added to the user set", user);

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}


#[debug_handler]
#[instrument]
pub async fn get_info(State(state): State<Arc<RwLock<Context>>>, Path(id): Path<String>) -> Response {
    event!(Level::DEBUG, "retrieving info for user with id {:?}", id);
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(error) => {
            event!(Level::WARN, "user info retrieval attempt for invalid user id format");
            return (StatusCode::BAD_REQUEST, format!("input id is not valid: {:?}", error)).into_response();
        }
    };

    let state = state.write().await;
    if let Some(user_mutex) = state.get_user(uuid).await {
        event!(Level::INFO, "user info retrieval for id {:?}", id);
        let user = user_mutex.lock().await;
        (StatusCode::OK, Json(user.clone())).into_response()
    } else {
        event!(Level::INFO, "user with id {:?} not found", id);
        (StatusCode::NOT_FOUND, format!("User with id {} is not found", id)).into_response()
    }
}
