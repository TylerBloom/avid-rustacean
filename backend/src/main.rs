use axum::{routing::get, Router, response::{Response, Html}, http::header, body::{Bytes, Body}};

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(landing))
        .route("/hourglass-frontend_bg.wasm", get(get_wasm))
        .route("/hourglass-frontend.js", get(get_js));

    Ok(router.into())
}

/* Bundle the frontend assets into the binary */
const INDEX_HTML: &str = include_str!("../../assets/index.html");
const APP_WASM: &[u8] = include_bytes!("../../assets/hourglass-frontend_bg.wasm");
const APP_JS: &str = include_str!("../../assets/hourglass-frontend.js");

/* Methods to get frontend assets */
pub async fn landing() -> Html<&'static str> {
    Html(INDEX_HTML)
}

pub async fn get_wasm() -> Response<Body> {
    let bytes = Bytes::from_static(APP_WASM);
    let body: Body = bytes.into();

    Response::builder()
        .header(header::CONTENT_TYPE, "application/wasm")
        .body(body)
        .unwrap()
}

pub async fn get_js() -> Response<Body> {
    let bytes = Bytes::from_static(APP_JS.as_bytes());
    let body: Body = bytes.into();

    Response::builder()
        .header(header::CONTENT_TYPE, "application/javascript;charset=utf-8")
        .body(body)
        .unwrap()
}
