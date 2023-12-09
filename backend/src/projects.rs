use avid_rustacean_model::Markdown;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct Project {
    name: String,
    summary: String,
    body: String,
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(Project { name, body, summary }): Json<Project>,
) -> (StatusCode, Json<Option<Markdown>>) {
    match body.parse::<Markdown>() {
        Ok(body) => {
            state.create_project(name, summary, body.clone());
            (StatusCode::OK, Json(Some(body)))
        }
        Err(_) => (StatusCode::BAD_REQUEST, Json(None)),
    }
}

pub async fn get_projects(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> (StatusCode, Json<Markdown>) {
    match state.get_project(&name) {
        Some(proj) => (StatusCode::OK, Json(proj)),
        None => (StatusCode::NOT_FOUND, Json(Markdown(Vec::new()))),
    }
}

pub async fn get_all_projects(
    State(state): State<AppState>,
) -> (StatusCode, Json<Vec<(String, String)>>) {
    (StatusCode::OK, Json(state.get_project_summaries()))
}
