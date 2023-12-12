use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use avid_rustacean_model::*;

#[derive(Debug, Clone)]
pub struct AppState {
    posts: Arc<RwLock<HashMap<String, Post>>>,
    projects: Arc<RwLock<HashMap<String, Project>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            posts: Arc::new(RwLock::new(HashMap::new())),
            projects: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Attempts to create a post and returns if it already exists.
    pub fn create_post(&self, post: Post) {
        self.posts
            .write()
            .unwrap()
            .insert(post.summary.title.clone(), post);
    }

    /// Attempts to create a post and returns if it already exists.
    pub fn create_project(&self, project: Project) {
        self.projects
            .write()
            .unwrap()
            .insert(project.summary.name.clone(), project);
    }

    /// Attempts to retrieve a post from the app state.
    pub fn get_project(&self, name: &str) -> Option<Project> {
        self.projects.read().unwrap().get(name).cloned()
    }

    /// Attempts to retrieve a post from the app state.
    pub fn get_post(&self, title: &str) -> Option<Post> {
        self.posts.read().unwrap().get(title).cloned()
    }

    pub fn get_project_summaries(&self) -> Vec<ProjectSummary> {
        self.projects
            .read()
            .unwrap()
            .values()
            .map(|proj| proj.summary.clone())
            .collect()
    }

    pub fn get_post_summaries(&self) -> Vec<PostSummary> {
        self.posts
            .read()
            .unwrap()
            .values()
            .map(|post| post.summary.clone())
            .collect()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
