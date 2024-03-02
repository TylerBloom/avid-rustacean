/*
<title>Without boats, dreams dry up</title>
<link>https://without.boats/</link>
<description>Recent content on Without boats, dreams dry up</description>
<generator>Hugo -- gohugo.io</generator>
<lastBuildDate>Sat, 24 Feb 2024 00:00:00 +0000</lastBuildDate>
<atom:link href="https://without.boats/index.xml" rel="self" type="application/rss+xml"/>
*/

use std::{fmt::Write, sync::Arc};

use avid_rustacean_model::{MdNode, Post};
use axum::{extract::State, response::Response};
use http::header::CONTENT_TYPE;
use rss::{Channel, ChannelBuilder, ItemBuilder};
use tracing::error;

use crate::{state::AppState, SERVER_ADDRESS};

pub async fn get_rss(State(state): State<AppState>) -> Response {
    let body = String::from(&*state.rss.read().unwrap().get_feed());
    Response::builder()
        .header(CONTENT_TYPE, "application/xml")
        .body(body.into())
        .unwrap()
}

#[derive(Debug, Clone)]
pub struct RssManager {
    channel: Channel,
    rss: Arc<str>,
}

impl RssManager {
    pub fn new() -> Self {
        let channel = ChannelBuilder::default()
            .title("The Avid Rustacean")
            .link("https://avid-rustacean.shuttleapp.rs/")
            .description("Content from the Avid Rustacean, including Rust from First Principles")
            .generator(Some("https://github.com/TylerBloom/avid-rustacean".into()))
            .build();
        Self {
            channel,
            rss: Arc::from(""),
        }
    }

    fn update_rss(&mut self) {
        let mut new_rss = Vec::new();
        if let Err(e) = self.channel.write_to(&mut new_rss) {
            error!("Failed to generate RSS doc! Got error: {e}");
        }
        match String::from_utf8(new_rss) {
            Ok(new_rss) => self.rss = Arc::from(new_rss),
            Err(e) => error!("RSS doc could not be converted to String! Got error: {e}"),
        }
    }

    pub fn load<'a>(&'a mut self, posts: impl Iterator<Item = &'a Post>) {
        for post in posts {
            self.add_post(post);
        }
        self.update_rss();
    }

    pub fn add_post(&mut self, post: &Post) {
        let mut builder = ItemBuilder::default();
        builder
            .title(Some(post.summary.title.clone()))
            .link(Some(format!(
                "{SERVER_ADDRESS}/blog/{}",
                url_escape::encode_path(&post.summary.title)
            )))
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
        self.channel.items.push(builder.build());
        self.update_rss();
    }

    pub fn get_feed(&self) -> Arc<str> {
        self.rss.clone()
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

impl Default for RssManager {
    fn default() -> Self {
        Self::new()
    }
}
