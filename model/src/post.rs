use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Markdown;

/// A container for all of the data needed for the backend to create a post.
/// (Only used by the blog-poster client).
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct CreatePost {
    pub title: String,
    pub summary: String,
    pub body: String,
}

/// A container for all of the data needed for a post
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Default)]
pub struct Post {
    pub summary: PostSummary,
    pub body: Markdown,
}

/// A container the summary of a post
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Default)]
pub struct PostSummary {
    pub title: String,
    pub summary: Markdown,
    pub create_on: DateTime<Utc>,
    pub last_edit: Option<DateTime<Utc>>,
}
