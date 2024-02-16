use gloo_net::http::Request;
use ratatui::prelude::*;
use serde::Deserialize;
use webatui::{backend::DehydratedSpan, WebTermMessage, WebTerminal};
use yew::prelude::*;

use crate::{
    app::{ScrollMotion, TermApp},
    utils::{Markdown, ScrollRef},
};

#[derive(Debug, PartialEq, Clone)]
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
    pub fn setup(&self, ctx: &Context<WebTerminal<TermApp>>) {
        let cp_name = self.title.clone().replace(' ', "-");
        ctx.link().send_future(async move {
            let post = match Request::get(&format!("/api/v1/posts/{cp_name}"))
                .send()
                .await
            {
                Ok(resp) => resp.json().await.unwrap_or_default(),
                Err(_) => avid_rustacean_model::Post::default(),
            };
            WebTermMessage::new(PostMessage::Post(post))
        });
    }

    pub fn create(name: String) -> Self {
        let mut real_name = String::with_capacity(name.len());
        url_escape::decode_to_string(name, &mut real_name);
        Self {
            title: real_name,
            body: Markdown::default(),
            scroll: 0,
        }
    }

    pub fn selected(&self) -> Option<usize> {
        None
    }

    pub fn hydrate(&self, ctx: &Context<WebTerminal<TermApp>>, span: &mut DehydratedSpan) {
        self.body.hydrate(ctx, span)
    }

    pub fn handle_scroll(&mut self, dir: ScrollMotion) {
        match dir {
            ScrollMotion::Up => self.scroll = self.scroll.saturating_add(1),
            ScrollMotion::Down => self.scroll = self.scroll.saturating_sub(1),
        }
    }

    pub fn update(&mut self, msg: PostMessage) {
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
