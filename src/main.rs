use std::sync::Arc;
use log::info;

use axum::{
    routing::{get, post},
    Router,
};
use routes::upload::upload;
use tokio::sync::RwLock;

mod routes;
mod models;
mod uuid;
use crate::routes::root::get_root;
use crate::routes::user::{create_user, get_info};
use crate::models::context::Context;


#[tokio::main]
async fn main() {
    env_logger::init();

    info!("starting up opulens");

    let root_router = Router::new()
        .route("/", get(get_root))
        .route("/user", post(create_user))
        .route("/upload", post(upload));

    let user_router = Router::new()
        .route("/info", get(get_info));

    let app: Router<()> = root_router.nest("/user/:id", user_router)
        .with_state(Arc::new(RwLock::new(Context::default())));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
