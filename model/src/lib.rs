use serde::{Deserialize, Serialize};

/// The parsed representation of markdown pages. The parsing occurs on the backend when it receives
/// a new project, blog post, or any updates to existing pages. Colorizing the markdown is task of
/// the frontend with the exception of syntax highlighting code blocks. Other formatting, such as
/// underlining and bolding, remains.
#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Markdown(pub Vec<MdNode>);

#[derive(Debug)]
pub struct MdError;

/// The supported markdown nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedCode(pub Vec<(String, (GruvboxColor, GruvboxColor))>);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum GruvboxColor {
    Neutral(GruvboxNeutral),
    Accent(GruvboxAccent),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum GruvboxNeutral {
    Dark(Shade),
    Light(Shade),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Shade {
    Darkest,
    Darker,
    Lighter,
    Lightest,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
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
pub use server::*;

#[cfg(feature = "server")]
mod server {
    use std::str::FromStr;

    use markdown::{mdast::Node, ParseOptions};
    use syntect::{
        dumps::from_binary,
        easy::HighlightLines,
        highlighting::{Color, Style},
        parsing::SyntaxSet,
    };

    use super::*;

    impl FromStr for Markdown {
        type Err = MdError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            fn process(node: Node, nodes: &mut Vec<MdNode>) -> Result<(), MdError> {
                if !matches!(node, Node::Root(_)) {
                    nodes.push((&node).try_into()?);
                }
                for node in node.children().iter().flat_map(|c| c.iter()) {
                    nodes.push(node.try_into()?);
                }
                Ok(())
            }

            println!("Attempting to parse markdown...");
            let Ok(ast) = markdown::to_mdast(s, &ParseOptions::gfm()) else {
                return Err(MdError);
            };
            let mut digest = Vec::new();
            process(ast, &mut digest)?;
            Ok(Self(digest))
        }
    }

    impl TryFrom<&Node> for MdNode {
        type Error = MdError;

        fn try_from(node: &Node) -> Result<Self, Self::Error> {
            println!("Attempting to process node: {node:?}");
            match node {
                Node::Paragraph(p) => p
                    .children
                    .iter()
                    .map(MdNode::try_from)
                    .collect::<Result<_, _>>()
                    .map(Self::Paragraph),
                Node::List(l) => l
                    .children
                    .iter()
                    .map(MdNode::try_from)
                    .collect::<Result<_, _>>()
                    .map(Self::List),
                Node::Text(n) => Ok(Self::Text(n.value.clone())),
                Node::ThematicBreak(_) => Ok(Self::ThematicBreak),
                Node::Break(_) => Ok(Self::Break),
                Node::Heading(h) => Ok(Self::Heading(nodes_to_string(&h.children))),
                Node::BlockQuote(b) => Ok(Self::BlockQuote(nodes_to_string(&b.children))),
                Node::InlineCode(c) => Ok(Self::InlineCode(c.value.clone())),
                Node::Emphasis(e) => Ok(Self::Emphasis(nodes_to_string(&e.children))),
                Node::Strong(s) => Ok(Self::Strong(nodes_to_string(&s.children))),
                Node::Link(l) => Ok(Self::Link(
                    l.title.clone().unwrap_or_else(|| l.url.clone()),
                    l.url.clone(),
                )),
                Node::Code(code) => parse_code(&code.value).map(Self::Code),
                _ => Err(MdError),
            }
        }
    }

    /// A loosely method for turning markdown nodes into strings.
    fn nodes_to_string(nodes: &[Node]) -> String {
        fn inner(acc: &mut String, nodes: &[Node]) {
            for node in nodes.iter() {
                if let Node::Text(txt) = node {
                    acc.push_str(&txt.value);
                } else if let Some(childern) = node.children() {
                    inner(acc, childern);
                }
            }
        }
        let mut digest = String::new();
        inner(&mut digest, nodes);
        digest
    }

    /// Parses a Rust code block and highlights the syntax.
    fn parse_code(code: &str) -> Result<ParsedCode, MdError> {
        static GRUVBYTES: &[u8] = include_bytes!("../assets/gruvbox.dump");
        let syntaxes = SyntaxSet::load_defaults_nonewlines();
        let theme = from_binary(GRUVBYTES);
        let mut hl = HighlightLines::new(syntaxes.find_syntax_by_name("Rust").unwrap(), &theme);
        let mut digest = Vec::new();
        for line in code.split_inclusive('\n') {
            let Ok(parsed) = hl.highlight_line(line, &syntaxes) else {
                return Err(MdError);
            };
            for (style, item) in parsed {
                digest.push((item.replace('\t', "  "), convert_style(style)?));
            }
        }
        Ok(ParsedCode(digest))
    }

    fn convert_style(style: Style) -> Result<(GruvboxColor, GruvboxColor), MdError> {
        let Ok(fg) = style.foreground.try_into() else {
            return Err(MdError);
        };
        let Ok(bg) = style.background.try_into() else {
            return Err(MdError);
        };
        Ok((fg, bg))
    }

    #[derive(Debug)]
    pub struct ColorError;

    impl TryFrom<Color> for GruvboxColor {
        type Error = ColorError;

        fn try_from(Color { r, g, b, .. }: Color) -> Result<Self, Self::Error> {
            match format!("#{r:x}{g:x}{b:x}").as_str() {
                BASE_0_HEX => Ok(GruvboxColor::dark_1()),
                BASE_1_HEX => Ok(GruvboxColor::dark_2()),
                BASE_2_HEX => Ok(GruvboxColor::dark_3()),
                BASE_3_HEX => Ok(GruvboxColor::dark_4()),
                BASE_4_HEX => Ok(GruvboxColor::light_1()),
                BASE_5_HEX => Ok(GruvboxColor::light_2()),
                BASE_6_HEX => Ok(GruvboxColor::light_3()),
                BASE_7_HEX => Ok(GruvboxColor::light_4()),
                BASE_8_HEX => Ok(GruvboxColor::red()),
                BASE_9_HEX => Ok(GruvboxColor::burnt_orange()),
                BASE_A_HEX => Ok(GruvboxColor::orange()),
                BASE_B_HEX => Ok(GruvboxColor::yellow()),
                BASE_C_HEX => Ok(GruvboxColor::green()),
                BASE_D_HEX => Ok(GruvboxColor::teal()),
                BASE_E_HEX => Ok(GruvboxColor::blue()),
                BASE_F_HEX => Ok(GruvboxColor::pink()),
                _ => Err(ColorError),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::parse_code;

        #[test]
        fn basic_code_parse() {
            parse_code("pub struct HelloWorld;").unwrap();
        }
    }
}
