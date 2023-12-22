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

use std::sync::OnceLock;

use axum::{async_trait, extract::FromRequestParts, routing::*, Json, Router};
use http::{request::Parts, StatusCode};
use mongodb::Database;
use serde::Serialize;
use sha2::Digest;

#[cfg(not(debug_assertions))]
pub mod assets;
pub mod home;
pub mod posts;
pub mod projects;
pub mod state;

use home::*;
use posts::*;
use projects::*;
use shuttle_secrets::SecretStore;
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
}

async fn badge_api() -> Json<Badge> {
    Json(Badge {
        schema_version: 1,
        label: "Deployment",
        message: "Active",
    })
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::MongoDb] db_conn: Database,
    #[shuttle_secrets::Secrets] _secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    /*
    API_KEY
        .set(
            secret_store
                .get("API_KEY")
                .expect("API_KEY not found in secrets!!"),
        )
        .unwrap();
    */

    let state = AppState::new(db_conn);
    state.load().await;

    let app = Router::new()
        // Homepage-related routes
        .route("/api/v1/badge", get(badge_api))
        // Homepage-related routes
        .route("/api/v1/home", post(update_homepage))
        .route("/api/v1/home", get(get_homepage))
        // Post-related routes
        .route("/api/v1/posts", post(create_post))
        .route("/api/v1/posts", get(get_post_summaries))
        .route("/api/v1/posts/:name", get(get_post))
        .route("/api/v1/posts/:name", delete(delete_post))
        // Project-related routes
        .route("/api/v1/projects", post(create_project))
        .route("/api/v1/projects", get(get_all_projects))
        .route("/api/v1/projects/:name", get(get_projects));

    #[cfg(not(debug_assertions))]
    let app = assets::inject_ui(app);

    Ok(app.layer(CorsLayer::permissive()).with_state(state).into())
}

/// The is a ZST used to signal that an API needs to be authorized. To do so, the [`API_KEY`] must
/// be present in the `Authorization` header.
pub struct AccessGaurd;

#[async_trait]
impl FromRequestParts<AppState> for AccessGaurd {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        if let Some(true) = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .map(check_auth_header)
        {
            Ok(Self)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

fn check_auth_header(header: &str) -> bool {
    // The hashed API key
    const KEY: [u8; 32] =
        hex_literal::hex!("445929267209c034d1e324834c17e0c8305df3dcb21d1710a639ac6ca08c648b");
    // Hash the header
    let mut hasher = sha2::Sha256::new();
    hasher.update(header);
    KEY[..] == hasher.finalize()[..]
}
