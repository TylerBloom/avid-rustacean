use axum::{
    body::{Body, Bytes},
    http::{header, Response},
    response::Html,
};

/* Bundle the frontend assets into the binary */
const INDEX_HTML: &str = include_str!("../../assets/index.html");
const APP_WASM: &[u8] = include_bytes!("../../assets/avid-rustacean-frontend_bg.wasm");
const APP_JS: &str = include_str!("../../assets/avid-rustacean-frontend.js");

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
