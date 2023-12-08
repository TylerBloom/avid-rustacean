use avid_rustacean_model::Markdown;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct Project {
    name: String,
    body: String,
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(Project { name, body }): Json<Project>,
) -> (StatusCode, Json<Option<Markdown>>) {
    match body.parse::<Markdown>() {
        Ok(body) => {
            state.create_project(name, body.clone());
            (StatusCode::OK, Json(Some(body)))
        }
        Err(_) => (StatusCode::BAD_REQUEST, Json(None)),
    }
}

pub async fn get_projects(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> (StatusCode, Json<Markdown>) {
    match state.get_project(&name) {
        Some(proj) => (StatusCode::OK, Json(proj)),
        None => (StatusCode::NOT_FOUND, Json(Markdown(Vec::new()))),
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
        ("Avid Rustacean", "The blog that you're reading right now!!"),
    ];
    let data = (0..100).map(|i| (format!("{}", data[i % 4].0), data[i % 4].1.to_owned()));
    (StatusCode::OK, Json(data.collect()))
}
