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

        let ast = markdown::to_mdast(s, &ParseOptions::gfm()).map_err(|e| e.to_string())?;
        let mut digest = Vec::new();
        process(ast, &mut digest)?;
        Ok(Self(digest))
    }
}

impl TryFrom<&Node> for MdNode {
    type Error = MdError;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
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
            Node::Blockquote(b) => Ok(Self::BlockQuote(nodes_to_string(&b.children))),
            Node::InlineCode(c) => Ok(Self::InlineCode(c.value.clone())),
            Node::Emphasis(e) => Ok(Self::Emphasis(nodes_to_string(&e.children))),
            Node::Strong(s) => Ok(Self::Strong(nodes_to_string(&s.children))),
            Node::Link(l) => {
                let text = l.children.first().and_then(|n| match n {
                    Node::Text(t) => Some(t.value.clone()),
                    _ => None,
                });
                Ok(Self::Link(
                    text.unwrap_or_else(|| l.url.clone()),
                    l.url.clone(),
                ))
            }
            Node::Code(code) => parse_code(&code.value).map(Self::Code),
            Node::Html(_) => Ok(Self::Paragraph(Vec::new())),
            node => Err(MdError::from(format!("Unsupported node type: {node:?}"))),
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
        let parsed = hl.highlight_line(line, &syntaxes)?;
        for (style, item) in parsed {
            digest.push((item.replace('\t', "  "), convert_style(style)?));
        }
    }
    Ok(ParsedCode(digest))
}

fn convert_style(style: Style) -> Result<(GruvboxColor, GruvboxColor), MdError> {
    let fg = if let Ok(fg) = style.foreground.try_into() {
        fg
    } else if style.foreground
        == (Color {
            r: 146,
            g: 131,
            b: 116,
            a: 255,
        })
    {
        GruvboxColor::orange()
    } else {
        return Err("Style error".into());
    };
    let Ok(bg) = style.background.try_into() else {
        return Err("Style error".into());
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
