use ratatui::prelude::*;
use serde::Deserialize;
use yew::prelude::*;

use crate::{
    app::TermApp,
    terminal::DehydratedSpan,
    utils::{Markdown, ScrollRef},
    HOST_ADDRESS,
};

#[derive(Debug, PartialEq)]
pub struct Post {
    title: String,
    body: Markdown,
    scroll: u16,
}

#[derive(Debug, PartialEq)]
pub enum PostMessage {
    Post(avid_rustacean_model::Post),
}

#[derive(Debug, PartialEq, Deserialize, Default, Clone)]
pub struct PostData {
    body: String,
}

impl Post {
    pub fn create(name: String, ctx: &Context<TermApp>) -> Self {
        let mut real_name = String::with_capacity(name.len());
        url_escape::decode_to_string(name, &mut real_name);
        let cp_name = real_name.clone();
        ctx.link().send_future(async move {
            let post =
                match reqwest::get(format!("http{HOST_ADDRESS}/api/v1/posts/{cp_name}")).await {
                    Ok(resp) => resp.json().await.unwrap_or_default(),
                    Err(_) => avid_rustacean_model::Post::default(),
                };
            PostMessage::Post(post)
        });
        Self {
            title: real_name,
            body: Markdown::default(),
            scroll: 0,
        }
    }

    pub fn selected(&self) -> Option<usize> {
        None
    }

    pub fn hydrate(&self, ctx: &Context<TermApp>, span: &mut DehydratedSpan) {
        self.body.hydrate(ctx, span)
    }

    pub fn handle_scroll(&mut self, dir: bool) {
        if dir {
            self.scroll = self.scroll.saturating_add(1);
        } else {
            self.scroll = self.scroll.saturating_sub(1);
        }
    }

    pub fn update(&mut self, _ctx: &Context<TermApp>, msg: PostMessage) {
        match msg {
            PostMessage::Post(post) => {
                self.body = Markdown::new(post.summary.title.clone(), post.body);
            }
        }
    }

    pub fn draw(&self, scroll: &ScrollRef, rect: Rect, frame: &mut Frame<'_>) {
        self.body.draw(scroll, rect, frame)
    }
}
