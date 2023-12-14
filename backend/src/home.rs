use std::sync::Arc;

use avid_rustacean_model::*;
use axum::{extract::State, http::StatusCode, Json};
use tracing::info;

use crate::{state::AppState, AccessGaurd};

pub async fn update_homepage(
    AccessGaurd: AccessGaurd,
    State(state): State<AppState>,
    Json(UpdateHome { body }): Json<UpdateHome>,
) -> StatusCode {
    info!("Updating the homepage...");
    let Ok(body) = body.parse::<Markdown>() else {
        return StatusCode::BAD_REQUEST;
    };
    state.update_homepage(body).await;
    StatusCode::OK
}

pub async fn get_homepage(State(state): State<AppState>) -> Json<Arc<HomePage>> {
    info!("Getting homepage...");
    Json(state.get_homepage())
}
