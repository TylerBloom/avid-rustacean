use crate::state::AppState;
use axum::{
    body::{Body, Bytes},
    response::{Html, Response},
    routing::get,
    Router,
};
use http::{header, HeaderMap, HeaderValue, StatusCode};

const INDEX_HTML: &str = include_str!("../../assets/index.html");
const APP_WASM: &[u8] = include_bytes!("../../assets/avid-rustacean-frontend_bg.wasm.gz");
const APP_JS: &str = include_str!("../../assets/avid-rustacean-frontend.js");

pub fn inject_ui(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/", get(landing))
        .route("/avid-rustacean-frontend_bg.wasm", get(get_wasm))
        .route("/avid-rustacean-frontend.js", get(get_js))
        .fallback(landing)
}

async fn landing() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn get_wasm() -> Response<Body> {
    let bytes = Bytes::copy_from_slice(APP_WASM);
    let body: Body = bytes.into();

    Response::builder()
        .header(header::CONTENT_ENCODING, "gzip") // Unzips the compressed file
        .header(header::CONTENT_TYPE, "application/wasm")
        .body(body)
        .unwrap()
}

async fn get_js() -> (StatusCode, HeaderMap, &'static str) {
    let mut headers = HeaderMap::with_capacity(1);
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/javascript;charset=utf-8"),
    );
    (StatusCode::OK, headers, APP_JS)
}
