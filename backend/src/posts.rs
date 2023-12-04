use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct Post {
    pub title: String,
    pub body: String,
}

pub async fn create_post(State(state): State<AppState>, Json(post): Json<Post>) -> StatusCode {
    if state.create_post(post) {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(title): Path<String>,
) -> (StatusCode, Json<Option<Post>>) {
    let body = state.get_post(&title);
    let status = match &body {
        Some(_) => StatusCode::OK,
        None => StatusCode::NOT_FOUND,
    };

    (status, Json(body))
}

pub async fn get_post_summaries(
    State(state): State<AppState>,
) -> (StatusCode, Json<Vec<(String, String)>>) {
    let body = state.get_post_summaries();
    (StatusCode::OK, Json(body))
}
