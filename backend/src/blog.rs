use avid_rustacean_model::blog::{BlogPost, PagerQuery, SummaryPager};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::state::AppState;

/// Returns a paginated view of blog posts summaries
pub async fn get_summary_pager(
    State(state): State<AppState>,
    Query(query): Query<PagerQuery>,
) -> Json<SummaryPager> {
    Json(state.get_pager(query).await)
}

/// Creates a blog post
pub async fn create_post(
    State(state): State<AppState>,
    Json(post): Json<(String, String)>,
) -> Json<Uuid> {
    Json(state.create_post(post).await)
}

/// Updates a particular blog post
pub async fn update_post(
    State(state): State<AppState>,
    Path(p_id): Path<Uuid>,
    Json(post): Json<(String, String)>,
) -> StatusCode {
    if state.update_post(p_id, post).await {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

/// Publishes a blog post
pub async fn publish_post(State(state): State<AppState>, Path(p_id): Path<Uuid>) -> StatusCode {
    if state.publish_post(p_id).await {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

/// Retrieves a particular blog post
pub async fn get_post(
    State(state): State<AppState>,
    Path(p_id): Path<Uuid>,
) -> Json<Option<BlogPost>> {
    Json(state.get_post(p_id).await)
}
