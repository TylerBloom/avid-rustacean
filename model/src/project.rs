use serde::{Deserialize, Serialize};

use crate::Markdown;

/// A container for all of the data needed for the backend to create a post.
/// (Only used by the blog-poster client).
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct CreateProject {
    pub name: String,
    pub summary: String,
    pub body: String,
}

/// A container for all of the data needed for a project
#[derive(Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct Project {
    pub summary: ProjectSummary,
    pub body: Markdown,
}

/// A container the summary of a project
#[derive(Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct ProjectSummary {
    pub name: String,
    pub summary: Markdown,
}
