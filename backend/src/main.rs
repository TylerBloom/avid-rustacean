#![allow(unused, dead_code)]

use axum::{
    routing::{get, patch, post, put},
    Router,
};
use mongodb::Database;

pub mod app;
pub mod posts;
pub mod state;
pub mod projects;

use posts::*;
use projects::*;
use state::AppState;
use tower_http::cors::CorsLayer;

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::MongoDb] db_conn: Database) -> shuttle_axum::ShuttleAxum {
    let state = AppState::new();

    let app = Router::new()
        .route("/api/v1/posts", post(create_post))
        .route("/api/v1/posts", get(get_post_summaries))
        .route("/api/v1/posts/:name", get(get_post))
        .route("/api/v1/projects", get(get_all_projects))
        .route("/:post", get(app::get_post))
        .layer(CorsLayer::permissive())
        .with_state(state)
        .into();

    Ok(app)
}
