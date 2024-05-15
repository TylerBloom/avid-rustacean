use std::collections::HashMap;

use avid_rustacean_model::*;
use chrono::DateTime;
use serde::Deserialize;

use crate::rss::RssManager;

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
    pub rss: RssManager,
    pub home: &'static HomePage,
    posts: HashMap<String, &'static Post>,
    pub post_sums: &'static [PostSummary],
    projects: HashMap<String, &'static Project>,
    pub proj_sums: &'static [ProjectSummary],
}

impl AppState {
    pub fn new() -> Self {
        // TODO: Remove the load method and parse everything here
        // Home page
        let home = Box::leak(Box::new(
            HomePage::new(include_str!("../../content/home.md")).unwrap(),
        ));

        // Blog
        let file_posts: HashMap<String, FileData> =
            toml::from_str(include_str!("../../content/posts.toml")).unwrap();

        let posts: HashMap<_, _> = file_posts
            .into_iter()
            .map(|(name, post)| (name, &*Box::leak::<'static>(Box::new(post.into_post()))))
            .collect();
        let mut sums = posts
            .values()
            .map(|p| p.summary.clone())
            .collect::<Vec<_>>();
        sums.sort_by(|a, b| a.create_on.cmp(&b.create_on));
        let post_sums = sums.leak();

        // Projects
        let file_projects: HashMap<String, FileData> =
            toml::from_str(include_str!("../../content/projects.toml")).unwrap();
        let projects: HashMap<_, _> = file_projects
            .into_iter()
            .map(|(name, proj)| (name, &*Box::leak::<'static>(Box::new(proj.into_project()))))
            .collect();
        let proj_sums = projects
            .values()
            .map(|p| p.summary.clone())
            .collect::<Vec<_>>()
            .leak();

        let rss = RssManager::new(posts.values().copied());
        Self {
            home,
            posts,
            projects,
            post_sums,
            proj_sums,
            rss,
        }
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
