use avid_rustacean_model::Markdown;
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
    pub summary: String,
    pub body: String,
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(Post {
        title,
        summary,
        body,
    }): Json<Post>,
) -> (StatusCode, Json<Option<Markdown>>) {
    match body.parse::<Markdown>() {
        Ok(body) => {
            state.create_post(title, summary, body.clone());
            (StatusCode::OK, Json(Some(body)))
        }
        Err(_) => (StatusCode::BAD_REQUEST, Json(None)),
    }
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(title): Path<String>,
) -> (StatusCode, Json<Markdown>) {
    println!("Attempting to fetch post with title: {title:?}");
    match state.get_post(&title) {
        Some(md) => (StatusCode::OK, Json(md)),
        None => (StatusCode::NOT_FOUND, Json(Markdown::default())),
    }
}

pub async fn get_post_summaries(
    State(state): State<AppState>,
) -> (StatusCode, Json<Vec<(String, String)>>) {
    let body = state.get_post_summaries();
    (StatusCode::OK, Json(body))
}
