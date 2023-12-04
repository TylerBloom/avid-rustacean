use axum::{extract::State, http::StatusCode, Json};

use crate::state::AppState;

pub async fn get_all_projects(
    State(_state): State<AppState>,
) -> (StatusCode, Json<Vec<(String, String)>>) {
    println!("Got projects request!");
    let data = vec![
        (
            "Squire".into(),
            "The tournament service that in pure Rust.".into(),
        ),
        (
            "SquireBot".into(),
            "The progenitor of Squire and the starting point of my Rust journey.".into(),
        ),
        (
            "Troupe".into(),
            "An actor library that I created from my work with Squire.".into(),
        ),
    ];
    (StatusCode::OK, Json(data))
}
