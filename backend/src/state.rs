use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use avid_rustacean_model::Markdown;
use serde::{Deserialize, Serialize};

use crate::posts::Post;

#[derive(Debug, Clone)]
pub struct AppState {
    posts: Arc<RwLock<HashMap<String, (String, Markdown)>>>,
    projects: Arc<RwLock<HashMap<String, Markdown>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            posts: Arc::new(RwLock::new(HashMap::new())),
            projects: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Attempts to create a post and returns if it already exists.
    pub fn create_post(&self, title: String, summary: String, body: Markdown) {
        self.posts
            .write()
            .unwrap()
            .insert(title.clone(), (summary, body));
    }

    /// Attempts to create a post and returns if it already exists.
    pub fn create_project(&self, name: String, body: Markdown) {
        self.projects.write().unwrap().insert(name, body);
    }

    /// Attempts to retrieve a post from the app state.
    pub fn get_project(&self, name: &str) -> Option<Markdown> {
        self.projects.read().unwrap().get(name).cloned()
    }

    /// Attempts to retrieve a post from the app state.
    pub fn get_post(&self, title: &str) -> Option<Markdown> {
        self.posts
            .read()
            .unwrap()
            .get(title)
            .map(|(_, md)| md.clone())
    }

    pub fn get_post_summaries(&self) -> Vec<(String, String)> {
        self.posts
            .read()
            .unwrap()
            .iter()
            .map(|(t, (s, _))| (t.clone(), s.clone()))
            .collect()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
