use avid_rustacean_model::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tracing::info;

use crate::state::AppState;

pub async fn get_projects(State(state): State<AppState>, Path(name): Path<String>) -> Response {
    info!("Getting project: {name:?}");
    match state.get_project(&name) {
        Some(proj) => (StatusCode::OK, Json(proj)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn get_all_projects(
    State(state): State<AppState>,
) -> (StatusCode, Json<&'static [ProjectSummary]>) {
    info!("Get project summaries...");
    (StatusCode::OK, Json(state.proj_sums))
}
