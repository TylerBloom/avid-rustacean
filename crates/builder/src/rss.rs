use std::fmt::Write;

use avid_rustacean_model::{MdNode, Post};
use axum::{extract::State, response::Response};
use chrono::Utc;
use http::header::CONTENT_TYPE;
use rss::{ChannelBuilder, Guid, Item, ItemBuilder};

use crate::{state::AppState, SERVER_ADDRESS};

pub async fn get_rss(State(state): State<AppState>) -> Response {
    Response::builder()
        .header(CONTENT_TYPE, "application/xml")
        .body(state.rss.get_feed().into())
        .unwrap()
}

#[derive(Debug, Clone)]
pub struct RssManager {
    rss: &'static str,
}

fn post_to_item(post: &Post) -> Item {
    let mut builder = ItemBuilder::default();
    let link = format!(
        "{SERVER_ADDRESS}/blog/{}",
        post.summary.title.replace(' ', "-")
    );
    let guid = Guid {
        value: link.clone(),
        permalink: true,
    };
    builder
        .title(Some(post.summary.title.clone()))
        .guid(Some(guid))
        .link(Some(link))
        .pub_date(Some(post.summary.create_on.to_string()));
    let mut body = String::new();
    let mut header_count = 0;
    for section in post.body.0.iter() {
        header_count += matches!(section, MdNode::Heading(_)) as u8;
        if header_count >= 3 {
            break;
        } else {
            let _ = markdown_to_html(section, &mut body);
        }
    }
    builder.description(Some(body));
    builder.build()
}

impl RssManager {
    pub fn new<'a>(posts: impl Iterator<Item = &'a Post>) -> Self {
        let mut channel = ChannelBuilder::default()
            .title("The Avid Rustacean")
            .link(SERVER_ADDRESS)
            .description("Content from the Avid Rustacean, including Rust from First Principles")
            .generator(Some("https://github.com/TylerBloom/avid-rustacean".into()))
            .build();

        channel.items.extend(posts.map(post_to_item));

        let mut rss = Vec::new();
        channel.set_last_build_date(Some(Utc::now().to_rfc3339()));
        if let Err(e) = channel.write_to(&mut rss) {
            panic!("Failed to generate RSS doc! Got error: {e}");
        }
        let rss = match String::from_utf8(rss) {
            Ok(rss) => rss.leak(),
            Err(e) => panic!("RSS doc could not be converted to String! Got error: {e}"),
        };
        Self { rss }
    }

    pub fn get_feed(&self) -> &'static str {
        self.rss
    }
}

fn markdown_to_html(node: &MdNode, html: &mut String) -> Result<(), std::fmt::Error> {
    match node {
        MdNode::Paragraph(nodes) => {
            html.push_str("<p>");
            for node in nodes {
                markdown_to_html(node, html)?;
            }
            html.push_str("</p>");
        }
        MdNode::List(nodes) => {
            html.push_str("<ul>");
            for node in nodes {
                markdown_to_html(node, html)?;
            }
            html.push_str("</ul>");
        }
        MdNode::Emphasis(text) => write!(html, "<em>{text}<em>")?,
        MdNode::Link(text, link) => write!(html, r#"<a href="{link}">{text}</a>"#)?,
        MdNode::Strong(text) => write!(html, "<strong>{text}</strong>")?,
        MdNode::Heading(text) => write!(html, "<h2>{text}<h2>")?,
        MdNode::Text(text) => html.push_str(text),
        MdNode::ThematicBreak => html.push_str("<br/>"),
        MdNode::Break => html.push_str("<br/>"),
        MdNode::Code(code) => {
            html.push_str("<pre>");
            for (snip, (fg, bg)) in code.0.iter() {
                write!(
                    html,
                    r#"<span style="color: {}; background-color: {}">{snip}</span>"#,
                    fg.hex_str(),
                    bg.hex_str()
                )?;
            }
            html.push_str("</pre>");
        }
        MdNode::BlockQuote(text) => write!(html, "<blockquote>{text}</blockquote>")?,
        MdNode::InlineCode(text) => write!(html, "<code>{text}</code>")?,
    }
    Ok(())
}
