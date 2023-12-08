#![allow(unused, dead_code)]

use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, StatusCode},
    response::{Html, Response},
    routing::{get, patch, post, put},
    Router,
};
use http::{header, HeaderValue};
use mongodb::Database;

pub mod posts;
pub mod projects;
pub mod state;

use posts::*;
use projects::*;
use state::AppState;
use tower_http::cors::CorsLayer;

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::MongoDb] db_conn: Database) -> shuttle_axum::ShuttleAxum {
    let state = AppState::new();

    let app = Router::new()
        // Post-related routes
        .route("/api/v1/posts", post(create_post))
        .route("/api/v1/posts", get(get_post_summaries))
        .route("/api/v1/posts/:name", get(get_post))
        // Project-related routes
        .route("/api/v1/projects", post(create_project))
        .route("/api/v1/projects", get(get_all_projects))
        .route("/api/v1/projects/:name", get(get_projects));

    #[cfg(not(debug_assertions))]
    let app = ui::inject_ui(app);

    let app = app.layer(CorsLayer::permissive()).with_state(state).into();

    Ok(app)
}

#[cfg(not(debug_assertions))]
mod ui {
    const INDEX_HTML: &str = include_str!("../../assets/index.html");
    const APP_WASM: &[u8] = include_bytes!("../../assets/avid-rustacean-frontend_bg.wasm");
    const APP_JS: &str = include_str!("../../assets/avid-rustacean-frontend.js");

    fn inject_ui(router: Router) -> Router {
        router
            .route("/", get(ui::landing))
            .route("/avid-rustacean-frontend_bg.wasm", get(ui::get_wasm))
            .route("/avid-rustacean-frontend.js", get(ui::get_js))
            .fallback(ui::landing)
    }

    pub async fn landing() -> Html<&'static str> {
        Html(INDEX_HTML)
    }

    pub async fn get_wasm() -> Response<Body> {
        let bytes = Bytes::copy_from_slice(APP_WASM);
        let body: Body = bytes.into();

        Response::builder()
            /*
            .header(header::CONTENT_ENCODING, "gzip") // Unzips the compressed file
            */
            .header(header::CONTENT_TYPE, "application/wasm")
            .body(body)
            .unwrap()
    }

    pub async fn get_js() -> (StatusCode, HeaderMap, &'static str) {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/javascript;charset=utf-8"),
        );
        (StatusCode::OK, headers, APP_JS)
    }
}
