use std::{collections::HashMap, path::PathBuf};

use avid_rustacean_model::*;
use chrono::DateTime;
use serde::Deserialize;

/// A struct used to store post data in files. This is used to store a post's data in a TOML file
/// so that it can be diff by git. The AppState uses this model as its source of truth on start up.
/// Doing so allows corrections by readers to be made via a PR and incorporated without needing to
/// go through the post creation API.
#[derive(Debug, Deserialize)]
struct FileData {
    title: String,
    summary: String,
    body: String,
    created: String,
}

impl FileData {
    fn into_post(self) -> Post {
        let body = self.body.parse().unwrap();
        let summary = self.summary.parse().unwrap();
        let summary = PostSummary {
            title: self.title,
            summary,
            create_on: DateTime::parse_from_rfc3339(&self.created).unwrap().into(),
            last_edit: None,
        };
        Post { summary, body }
    }

    fn into_project(self) -> Project {
        let body = self.body.parse().unwrap();
        let summary = self.summary.parse().unwrap();
        let summary = ProjectSummary {
            name: self.title,
            summary,
        };
        Project { summary, body }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub home: &'static HomePage,
    posts: HashMap<String, &'static Post>,
    pub post_sums: &'static [PostSummary],
    projects: HashMap<String, &'static Project>,
    pub proj_sums: &'static [ProjectSummary],
}

impl AppState {
    pub fn new() -> Self {
        // Collect markdown docs
        let mut assets_path: PathBuf = env!("CARGO_MANIFEST_DIR").parse().unwrap();
        assets_path.pop();
        assets_path.push("assets");
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

        // Projects page
        let data = md_assets.remove("projects.md").unwrap();
        let (_, md) = split_markdown(&data);
        let projects: Markdown = md.parse().unwrap();

        // Blog
        let mut posts = HashMap::new();

        for (path, data) in md_assets {
            let path = path.replace(".md", ".json");
            let (metadata, md) = split_markdown(&data);
            let PostMetadata {
                title,
                date,
                description,
            } = toml::from_str(&metadata).unwrap();
            let post = Post {
                summary: PostSummary {
                    title,
                    summary: description.parse().unwrap(),
                    create_on: date,
                    last_edit: None,
                },
                body: md.parse().unwrap(),
            };
            posts.insert(path, post);
        }
        todo!()
    }

    /// Attempts to retrieve a post from the app state.
    pub fn get_post(&self, title: &str) -> Option<&'static Post> {
        self.posts.get(title).copied()
    }

    /// Attempts to retrieve a post from the app state.
    pub fn get_project(&self, name: &str) -> Option<&'static Project> {
        self.projects.get(name).copied()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::AppState;

    #[test]
    fn load_state() {
        let state = AppState::new();
        assert!(!state.home.body.0.is_empty());
        assert!(!state.posts.is_empty());
        assert_eq!(state.post_sums.len(), state.posts.len());
        assert!(!state.projects.is_empty());
        assert_eq!(state.proj_sums.len(), state.projects.len());
    }
}
