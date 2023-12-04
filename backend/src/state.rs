use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

use crate::posts::Post;

#[derive(Debug, Clone)]
pub struct AppState {
    posts: Arc<RwLock<HashMap<String, Post>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            posts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Attempts to create a post and returns if it already exists.
    pub fn create_post(&self, post: Post) -> bool {
        self.posts
            .write()
            .unwrap()
            .insert(post.title.clone(), post)
            .is_none()
    }

    /// Attempts to retrieve a post from the app state.
    pub fn get_post(&self, title: &str) -> Option<Post> {
        self.posts.read().unwrap().get(title).cloned()
    }

    pub fn get_post_summaries(&self) -> Vec<(String, String)> {
        self.posts
            .read()
            .unwrap()
            .values()
            .map(|post| (post.title.clone(), post.body.clone()))
            .collect()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
