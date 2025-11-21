use avid_rustacean_model::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tracing::info;

use crate::state::AppState;

pub async fn get_post(State(state): State<AppState>, Path(title): Path<String>) -> Response {
    info!("Getting post: {title:?}");
    let title = title.replace('-', " ");
    match state.get_post(&title) {
        Some(post) => (StatusCode::OK, Json(post)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn get_post_summaries(
    State(state): State<AppState>,
) -> (StatusCode, Json<&'static [PostSummary]>) {
    info!("Get post summaries...");
    (StatusCode::OK, Json(state.post_sums))
}
