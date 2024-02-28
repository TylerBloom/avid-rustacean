use std::sync::Arc;

use avid_rustacean_model::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tracing::{error, info};

use crate::{state::AppState, AccessGaurd};

pub async fn create_post(
    AccessGaurd: AccessGaurd,
    State(state): State<AppState>,
    Json(CreatePost {
        title,
        summary,
        body,
    }): Json<CreatePost>,
) -> (StatusCode, Json<Markdown>) {
    info!("Creating post with title: {title:?}");
    let Ok(body) = body.parse::<Markdown>() else {
        error!("Failed to parse post body...");
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
    state.create_post(post).await;
    (StatusCode::OK, Json(body))
}

pub async fn get_post(State(state): State<AppState>, Path(title): Path<String>) -> Response {
    info!("Getting post: {title:?}");
    let title = title.replace('-', " ");
    match state.get_post(&title) {
        Some(post) => (StatusCode::OK, Json(post)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn delete_post(
    AccessGaurd: AccessGaurd,
    State(state): State<AppState>,
    Path(title): Path<String>,
) -> StatusCode {
    info!("Delete post: {title:?}");
    let title = title.replace('-', " ");
    if state.delete_post(&title).await {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn get_post_summaries(
    State(state): State<AppState>,
) -> (StatusCode, Json<Arc<Vec<PostSummary>>>) {
    info!("Get post summaries...");
    let body = state.get_post_summaries();
    (StatusCode::OK, Json(body))
}
