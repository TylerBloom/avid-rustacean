#![warn(rust_2018_idioms)]
#![deny(
    rustdoc::broken_intra_doc_links,
    unreachable_pub,
    unreachable_patterns,
    unused,
    unused_qualifications,
    while_true,
    trivial_casts,
    trivial_bounds,
    trivial_numeric_casts,
    unconditional_panic,
    clippy::all
)]

use std::{collections::HashMap, path::PathBuf};

use avid_rustacean_model::{split_markdown, HomePage, Markdown, Post, PostSummary};
use toml::Value;

fn main() {
    // Path to assets directory
    let mut assets_path: PathBuf = env!("CARGO_MANIFEST_DIR").parse().unwrap();
    assets_path.pop();
    assets_path.push("assets");

    if !assets_path.exists() {
        std::fs::create_dir(&assets_path).unwrap();
    }

    // Generate badge json
    let json = r#"
    {
        "schemaVersion": 1,
        "label": "Deployment",
        "message": "Active",
        "color": "8ec07c"
    }"#;
    assets_path.push("badge.json");
    std::fs::write(&assets_path, json).unwrap();
    assets_path.pop();

    // Collect markdown docs
    let mut md_assets: HashMap<String, String> = std::fs::read_dir(&assets_path)
        .unwrap()
        .map(Result::unwrap)
        .filter_map(|file| {
            let name = file
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            name.ends_with(".md").then(|| {
                let data = std::fs::read_to_string(file.path()).unwrap();
                (name, data)
            })
        })
        .collect();
    md_assets.remove("_index.md");

    // Home page
    let data = md_assets.remove("home.md").unwrap();
    let (_, md) = split_markdown(&data);
    let home = HomePage {
        body: md.parse().unwrap(),
    };
    let json = serde_json::to_string(&home).unwrap();
    assets_path.push("home.json");
    std::fs::write(&assets_path, &json).unwrap();
    assets_path.pop();

    // Projects page
    let data = md_assets.remove("projects.md").unwrap();
    let (_, md) = split_markdown(&data);
    let projects: Markdown = md.parse().unwrap();
    let json = serde_json::to_string(&projects).unwrap();
    assets_path.push("projects.json");
    std::fs::write(&assets_path, &json).unwrap();
    assets_path.pop();

    // Blog
    let mut posts = Vec::new();
    for (path, data) in md_assets {
        let path = path.replace(".md", ".json");
        let (metadata, md) = split_markdown(&data);
        let value: Value = toml::from_str(&metadata).unwrap();
        let Value::Table(table) = value else { panic!() };
        let Value::Datetime(created_on) = table.get("date").unwrap() else {
            panic!()
        };
        let create_on = created_on.to_string();
        let summary = PostSummary {
            title: table.get("title").unwrap().to_string().clone(),
            real_name: path.split_once(".json").unwrap().0.to_string(),
            summary: table
                .get("description")
                .unwrap()
                .to_string()
                .replace(r#"""""#, "")
                .replace(r#"'''"#, "")
                .parse()
                .unwrap(),
            create_on,
            last_edit: None,
        };
        posts.push((*created_on, summary.clone()));
        let post = Post {
            summary,
            body: md.parse().unwrap(),
        };
        let json = serde_json::to_string(&post).unwrap();
        assets_path.push(path);
        std::fs::write(&assets_path, &json).unwrap();
        assets_path.pop();
    }
    posts.sort_by(|prior, next| prior.0.cmp(&next.0));
    let posts = posts
        .into_iter()
        .map(|(_, summary)| summary)
        .collect::<Vec<_>>();
    let json = serde_json::to_string(&posts).unwrap();
    assets_path.push("posts.json");
    std::fs::write(&assets_path, &json).unwrap();
    assets_path.pop();
}
