use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::state::AppState;

pub async fn create_project(State(_state): State<AppState>) -> StatusCode {
    StatusCode::OK
}

pub async fn get_projects(
    State(_state): State<AppState>,
    Path(name): Path<String>,
) -> (StatusCode, Json<String>) {
    match name.as_str() {
        "Squire" => (
            StatusCode::OK,
            Json("The tournament service that in pure Rust.".into()),
        ),
        "SquireBot" => (
            StatusCode::OK,
            Json("The progenitor of Squire and the starting point of my Rust journey.".into()),
        ),
        "Troupe" => (
            StatusCode::OK,
            Json("An actor library that I created from my work with Squire.".into()),
        ),
        _ => (StatusCode::NOT_FOUND, Json(String::new())),
    }
}

pub async fn get_all_projects(
    State(_state): State<AppState>,
) -> (StatusCode, Json<Vec<(String, String)>>) {
    println!("Got projects request!");
    let data = [
        ("Squire", "The tournament service that in pure Rust."),
        (
            "SquireBot",
            "The progenitor of Squire and the starting point of my Rust journey.",
        ),
        (
            "Troupe",
            "An actor library that I created from my work with Squire.",
        ),
    ]
    .repeat(20);
    (
        StatusCode::OK,
        Json(
            data.into_iter()
                .map(|(t, b)| (t.to_owned(), b.to_owned()))
                .collect(),
        ),
    )
}
