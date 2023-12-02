use axum::{
    extract::{Path, State},
    response::Html, http::StatusCode,
};

use crate::state::AppState;

pub async fn get_post(State(state): State<AppState>, Path(title): Path<String>) -> (StatusCode, Html<String>) {
    match state.get_post(&title) {
        Some(post) => (StatusCode::OK, Html(format!("<h1>{}</h1><p>{}</p>",post.title, post.body))),
        None => (StatusCode::NOT_FOUND, Html(String::new())),
    }
}
