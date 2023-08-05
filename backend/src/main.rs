use avid_rustacean_model::api::{BLOG_API, BLOG_POST_API};
use axum::{
    routing::{get, patch, post, put},
    Router,
};

mod assets;
mod blog;
mod projects;
mod state;

use assets::*;
use blog::*;
use mongodb::Database;
use state::*;

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::MongoDb] db_conn: Database) -> shuttle_axum::ShuttleAxum {
    let state = AppState::new(db_conn).await;

    let router = Router::new()
        .route(BLOG_API.as_str(), get(get_summary_pager))
        .route(BLOG_POST_API.as_str(), get(get_post))
        .route(BLOG_POST_API.as_str(), post(create_post))
        .route(BLOG_POST_API.as_str(), patch(publish_post))
        .route(BLOG_POST_API.as_str(), put(update_post))
        .route("/", get(landing))
        .route("/avid-rustacean-frontend_bg.wasm", get(get_wasm))
        .route("/avid-rustacean-frontend.js", get(get_js))
        .with_state(state);

    Ok(router.into())
}
