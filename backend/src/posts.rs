use avid_rustacean_model::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::state::AppState;

pub async fn create_post(
    State(state): State<AppState>,
    Json(CreatePost {
        title,
        summary,
        body,
    }): Json<CreatePost>,
) -> (StatusCode, Json<Markdown>) {
    let Ok(body) = body.parse::<Markdown>() else {
        return (StatusCode::BAD_REQUEST, Json(Markdown::default()));
    };
    let Ok(summary) = summary.parse::<Markdown>() else {
        return (StatusCode::BAD_REQUEST, Json(Markdown::default()));
    };
    let summary = PostSummary {
        title,
        summary,
        create_on: Utc::now(),
        last_edit: None,
    };
    let post = Post {
        summary,
        body: body.clone(),
    };
    state.create_post(post);
    (StatusCode::OK, Json(body))
}

pub async fn get_post(State(state): State<AppState>, Path(title): Path<String>) -> Response {
    match state.get_post(&title) {
        Some(post) => (StatusCode::OK, Json(post)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn get_post_summaries(
    State(state): State<AppState>,
) -> (StatusCode, Json<Vec<PostSummary>>) {
    let body = state.get_post_summaries();
    (StatusCode::OK, Json(body))
}
