use avid_rustacean_model::*;
use axum::{extract::State, Json};
use tracing::info;

use crate::state::AppState;

pub async fn get_homepage(State(state): State<AppState>) -> Json<&'static HomePage> {
    info!("Getting homepage...");
    Json(state.home)
}
