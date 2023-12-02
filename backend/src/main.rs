#![allow(unused, dead_code)]

use axum::{
    routing::{get, patch, post, put},
    Router,
};
use mongodb::Database;

pub mod app;
pub mod posts;
pub mod state;

use posts::*;
use state::AppState;

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::MongoDb] db_conn: Database) -> shuttle_axum::ShuttleAxum {
    let state = AppState::new();

    let app = Router::new()
        .route("/api/v1/posts", post(create_post))
        .route("/api/v1/posts", get(get_post))
        .route("/:post", get(app::get_post))
        .with_state(state)
        .into();

    Ok(app)
}
