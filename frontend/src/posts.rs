use std::sync::Mutex;

use js_sys::Function;
use ratatui::{prelude::*, widgets::*};
use serde::Deserialize;
use wasm_bindgen::prelude::Closure;
use yew::prelude::*;

use crate::{
    app::{CursorMap, TermApp},
    console_debug, console_log,
    terminal::{get_window_size, DehydratedSpan},
    utils::{render_markdown, Markdown, ScrollRef},
    HOST_ADDRESS, TERMINAL,
};

static SCROLL_STATE: Mutex<Option<ScrollbarState>> = Mutex::new(None);

#[derive(Debug, PartialEq)]
pub struct Post {
    title: String,
    body: Markdown,
    scroll: u16,
}

#[derive(Debug, PartialEq)]
pub enum PostMessage {
    Post(Markdown),
}

#[derive(Debug, PartialEq, Deserialize, Default, Clone)]
pub struct PostData {
    body: String,
}

impl Post {
    pub fn create(name: String, ctx: &Context<TermApp>, map: &mut CursorMap) -> Self {
        let mut real_name = String::with_capacity(name.len());
        url_escape::decode_to_string(name, &mut real_name);
        let cp_name = real_name.clone();
        ctx.link().send_future(async move {
            let post =
                match reqwest::get(format!("http{HOST_ADDRESS}/api/v1/posts/{cp_name}")).await {
                    Ok(resp) => Markdown::new(
                        cp_name,
                        resp.json::<avid_rustacean_model::Markdown>()
                            .await
                            .unwrap_or_default(),
                    ),
                    Err(e) => Markdown::default(),
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
        // TODO: Hydrate as needed...
    }

    pub fn handle_scroll(&mut self, dir: bool) {
        if dir {
            self.scroll = self.scroll.saturating_add(1);
        } else {
            self.scroll = self.scroll.saturating_sub(1);
        }
    }

    pub fn update(
        &mut self,
        ctx: &Context<TermApp>,
        msg: PostMessage,
        map: &mut CursorMap,
    ) {
        map.clear_after(1);
        match msg {
            PostMessage::Post(data) => {
                self.body = data;
            }
        }
    }

    pub fn draw(&self, scroll: &ScrollRef, mut rect: Rect, frame: &mut Frame) {
        self.body.draw(scroll, rect, frame)
    }
}
