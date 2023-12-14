use serde::{Deserialize, Serialize};

use crate::Markdown;

/// A container that contains all of the data needed to update the home page of the blog.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct UpdateHome {
    pub body: String,
}

/// A container for the pre-parsed markdown used to display the homepage.
#[derive(Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct HomePage {
    pub body: Markdown,
}
