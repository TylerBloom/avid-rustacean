use serde::{Deserialize, Serialize};

use crate::Markdown;

/// A container for the pre-parsed markdown used to display the homepage.
#[derive(Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct HomePage {
    pub body: Markdown,
}

impl HomePage {
    #[cfg(feature = "server")]
    pub fn new(md: &str) -> Option<Self> {
        let (_, md) = crate::split_markdown(md);
        md.parse().ok().map(|body| Self { body })
    }
}
