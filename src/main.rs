#![allow(dead_code)]

use log::info;
use std::sync::Arc;

use axum::{
    routing::{get, post, put},
    Router,
};
use routes::user::{get_user, update_user};
use tokio::sync::RwLock;

mod config;
mod connectors;
mod models;
mod routes;
mod service;
mod uuid;

use crate::config::Cli;
use crate::models::context::Context;
use crate::routes::user::create_user;
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let toml_config = cli
        .read_config()
        .expect("config is mandatory, failed to read config");

    // TODO: support basic logging with cli args
    log4rs::init_file(&toml_config.log_config, Default::default()).expect("no log config given");

    info!("loaded config {:?}", toml_config);
    info!("starting up opulens");

    let context = Context::try_from_config(&toml_config)
        .await
        .expect("unable to work without context, aborting");

    let root_router = Router::new()
        .route("/user", post(create_user))
        .route("/user", put(update_user));

    let user_router = Router::new().route("/info", get(get_user));

    let app: Router<()> = root_router
        .nest("/user/:id", user_router)
        .with_state(Arc::new(RwLock::new(context)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
