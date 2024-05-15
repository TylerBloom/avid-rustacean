#![warn(rust_2018_idioms)]
#![deny(
    rustdoc::broken_intra_doc_links,
    unreachable_pub,
    unreachable_patterns,
    unused,
    unused_qualifications,
    while_true,
    trivial_casts,
    trivial_bounds,
    trivial_numeric_casts,
    unconditional_panic,
    clippy::all
)]

/// The address of the server, used for the RSS feed.
pub static SERVER_ADDRESS: &str = "https://avid-rustacean.shuttleapp.rs";

use std::sync::OnceLock;

use axum::{routing::*, Json, Router};
use rss::get_rss;
use serde::Serialize;

#[cfg(not(debug_assertions))]
pub mod assets;
pub mod home;
pub mod posts;
pub mod projects;
pub mod rss;
pub mod state;

use home::*;
use posts::*;
use projects::*;
use state::AppState;
use tower_http::cors::CorsLayer;

pub static API_KEY: OnceLock<String> = OnceLock::new();

/// Returns data necessary for shields.io to construct a badge to monitor deployment
#[derive(Debug, Serialize)]
struct Badge {
    #[serde(rename(serialize = "schemaVersion"))]
    schema_version: usize,
    label: &'static str,
    message: &'static str,
    color: &'static str,
}

async fn badge_api() -> Json<Badge> {
    Json(Badge {
        schema_version: 1,
        label: "Deployment",
        message: "Active",
        color: "8ec07c",
    })
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let state = AppState::new();

    let app = Router::new()
        // Homepage-related routes
        .route("/api/v1/rss", get(get_rss))
        // Homepage-related routes
        .route("/api/v1/home", get(get_homepage))
        // Post-related routes
        .route("/api/v1/posts", get(get_post_summaries))
        .route("/api/v1/posts/:name", get(get_post))
        // Project-related routes
        .route("/api/v1/projects", get(get_all_projects))
        .route("/api/v1/projects/:name", get(get_projects))
        // Misc
        .route("/api/v1/badge", get(badge_api));

    #[cfg(not(debug_assertions))]
    let app = assets::inject_ui(app);

    Ok(app.layer(CorsLayer::permissive()).with_state(state).into())
}
