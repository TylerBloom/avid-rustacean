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

use chrono::DateTime;
pub use chrono::Utc;
use serde::{Deserialize, Serialize};

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

/// The parsed representation of markdown pages. The parsing occurs on the backend when it receives
/// a new project, blog post, or any updates to existing pages. Colorizing the markdown is task of
/// the frontend with the exception of syntax highlighting code blocks. Other formatting, such as
/// underlining and bolding, remains.
#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
pub struct Markdown(pub Vec<MdNode>);

#[derive(Debug)]
pub struct MdError;

/// The supported markdown nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MdNode {
    Paragraph(Vec<Self>),
    List(Vec<Self>),
    Code(ParsedCode),
    BlockQuote(String),
    InlineCode(String),
    Emphasis(String),
    Link(String, String),
    Strong(String),
    Heading(String),
    Text(String),
    ThematicBreak,
    Break,
}

/// Parsed and syntax highlights Rust code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ParsedCode(pub Vec<(String, (GruvboxColor, GruvboxColor))>);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum GruvboxColor {
    Neutral(GruvboxNeutral),
    Accent(GruvboxAccent),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum GruvboxNeutral {
    Dark(Shade),
    Light(Shade),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum Shade {
    Darkest,
    Darker,
    Lighter,
    Lightest,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum GruvboxAccent {
    Red,
    BurntOrange,
    Orange,
    Yellow,
    Green,
    Teal,
    Blue,
    Pink,
}

impl GruvboxColor {
    pub const fn default_fg() -> Self {
        Self::light_3()
    }

    pub const fn default_bg() -> Self {
        Self::dark_2()
    }

    pub const fn dark_1() -> Self {
        Self::Neutral(GruvboxNeutral::Dark(Shade::Darkest))
    }

    pub const fn dark_2() -> Self {
        Self::Neutral(GruvboxNeutral::Dark(Shade::Darker))
    }

    pub const fn dark_3() -> Self {
        Self::Neutral(GruvboxNeutral::Dark(Shade::Lighter))
    }

    pub const fn dark_4() -> Self {
        Self::Neutral(GruvboxNeutral::Dark(Shade::Lightest))
    }

    pub const fn light_1() -> Self {
        Self::Neutral(GruvboxNeutral::Light(Shade::Darkest))
    }

    pub const fn light_2() -> Self {
        Self::Neutral(GruvboxNeutral::Light(Shade::Darker))
    }

    pub const fn light_3() -> Self {
        Self::Neutral(GruvboxNeutral::Light(Shade::Lighter))
    }

    pub const fn light_4() -> Self {
        Self::Neutral(GruvboxNeutral::Light(Shade::Lightest))
    }

    pub const fn red() -> Self {
        Self::Accent(GruvboxAccent::Red)
    }

    pub const fn burnt_orange() -> Self {
        Self::Accent(GruvboxAccent::BurntOrange)
    }

    pub const fn orange() -> Self {
        Self::Accent(GruvboxAccent::Orange)
    }

    pub const fn yellow() -> Self {
        Self::Accent(GruvboxAccent::Yellow)
    }

    pub const fn green() -> Self {
        Self::Accent(GruvboxAccent::Green)
    }

    pub const fn teal() -> Self {
        Self::Accent(GruvboxAccent::Teal)
    }

    pub const fn blue() -> Self {
        Self::Accent(GruvboxAccent::Blue)
    }

    pub const fn pink() -> Self {
        Self::Accent(GruvboxAccent::Pink)
    }
}

// Darks
pub const BASE_0_HEX: &str = "#1d2021";
pub const BASE_1_HEX: &str = "#3c3836";
pub const BASE_2_HEX: &str = "#504945";
pub const BASE_3_HEX: &str = "#665c54";

// Lights
pub const BASE_4_HEX: &str = "#bdae93";
pub const BASE_5_HEX: &str = "#d5c4a1";
pub const BASE_6_HEX: &str = "#ebdbb2";
pub const BASE_7_HEX: &str = "#fbf1c7";

// Accents
pub const BASE_8_HEX: &str = "#fb4934";
pub const BASE_9_HEX: &str = "#d65d0e";
pub const BASE_A_HEX: &str = "#fe8019";
pub const BASE_B_HEX: &str = "#fabd2f";
pub const BASE_C_HEX: &str = "#b8bb26";
pub const BASE_D_HEX: &str = "#8ec07c";
pub const BASE_E_HEX: &str = "#83a598";
pub const BASE_F_HEX: &str = "#d3869b";

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "server")]
pub use server::*;
