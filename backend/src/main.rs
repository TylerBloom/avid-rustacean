#![allow(unused, dead_code)]

use axum::{
    routing::{get, patch, post, put},
    Router,
};
use mongodb::Database;

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
        // Post-related routes
        .route("/api/v1/posts", post(create_post))
        .route("/api/v1/posts", get(get_post_summaries))
        .route("/api/v1/posts/:name", get(get_post))
        // Project-related routes
        .route("/api/v1/projects", post(create_project))
        .route("/api/v1/projects", get(get_all_projects))
        .route("/api/v1/projects/:name", get(get_projects))
        .layer(CorsLayer::permissive())
        .with_state(state)
        .into();

    Ok(app)
}
