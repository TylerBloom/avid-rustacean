use std::sync::Arc;

use avid_rustacean_model::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tracing::info;

use crate::state::AppState;

pub async fn create_project(
    State(state): State<AppState>,
    Json(CreateProject {
        name,
        body,
        summary,
    }): Json<CreateProject>,
) -> Response {
    info!("Creating project with name: {name:?}");
    let Ok(body) = body.parse::<Markdown>() else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(summary) = summary.parse::<Markdown>() else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let summary = ProjectSummary { name, summary };
    let project = Project {
        summary,
        body: body.clone(),
    };
    state.create_project(project).await;
    (StatusCode::OK, Json(body)).into_response()
}

pub async fn get_projects(State(state): State<AppState>, Path(name): Path<String>) -> Response {
    info!("Getting project: {name:?}");
    match state.get_project(&name) {
        Some(proj) => (StatusCode::OK, Json(proj)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn get_all_projects(
    State(state): State<AppState>,
) -> (StatusCode, Json<Arc<Vec<ProjectSummary>>>) {
    info!("Get project summaries...");
    (StatusCode::OK, Json(state.get_project_summaries()))
}
