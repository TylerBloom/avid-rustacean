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

use std::error::Error;

pub use chrono::Utc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

mod home;
mod post;

pub use home::*;
pub use post::*;

pub fn split_markdown(file: &str) -> (String, String) {
    let mut lines = file.lines();

    // This should be the start of the metadata
    assert_eq!(lines.next(), Some("+++"));

    // The metadata is everything between the two lines that only contain '+++'
    let metadata = lines
        .by_ref()
        .take_while(|line| *line != "+++")
        .format("\n")
        .to_string();

    // Everything else should be markdown
    let md = lines.format("\n").to_string();
    (metadata, md)
}

/// The parsed representation of markdown pages. The parsing occurs on the backend when it receives
/// a new project, blog post, or any updates to existing pages. Colorizing the markdown is task of
/// the frontend with the exception of syntax highlighting code blocks. Other formatting, such as
/// underlining and bolding, remains.
#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
pub struct Markdown(pub Vec<MdNode>);

pub type MdError = Box<dyn Error>;

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

    pub const fn hex_str(&self) -> &'static str {
        match self {
            GruvboxColor::Neutral(GruvboxNeutral::Dark(Shade::Darkest)) => "#1d2021",
            GruvboxColor::Neutral(GruvboxNeutral::Dark(Shade::Darker)) => "#3c3836",
            GruvboxColor::Neutral(GruvboxNeutral::Dark(Shade::Lighter)) => "#504945",
            GruvboxColor::Neutral(GruvboxNeutral::Dark(Shade::Lightest)) => "#665c54",
            GruvboxColor::Neutral(GruvboxNeutral::Light(Shade::Darkest)) => "#bdae93",
            GruvboxColor::Neutral(GruvboxNeutral::Light(Shade::Darker)) => "#d5c4a1",
            GruvboxColor::Neutral(GruvboxNeutral::Light(Shade::Lighter)) => "#ebdbb2",
            GruvboxColor::Neutral(GruvboxNeutral::Light(Shade::Lightest)) => "#fbf1c7",
            GruvboxColor::Accent(GruvboxAccent::Red) => "#fb4934",
            GruvboxColor::Accent(GruvboxAccent::BurntOrange) => "#d65d0e",
            GruvboxColor::Accent(GruvboxAccent::Orange) => "#fe8019",
            GruvboxColor::Accent(GruvboxAccent::Yellow) => "#fabd2f",
            GruvboxColor::Accent(GruvboxAccent::Green) => "#b8bb26",
            GruvboxColor::Accent(GruvboxAccent::Teal) => "#8ec07c",
            GruvboxColor::Accent(GruvboxAccent::Blue) => "#83a598",
            GruvboxColor::Accent(GruvboxAccent::Pink) => "#d3869b",
        }
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
