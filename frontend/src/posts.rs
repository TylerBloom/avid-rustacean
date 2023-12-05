use std::sync::Mutex;

use js_sys::Function;
use ratatui::{prelude::*, widgets::*};
use wasm_bindgen::prelude::Closure;
use yew::prelude::*;

use crate::{
    app::{CursorMap, TermApp},
    console_debug, console_log,
    terminal::get_window_size,
    HOST_ADDRESS, TERMINAL,
};

static SCROLL_STATE: Mutex<Option<ScrollbarState>> = Mutex::new(None);

#[derive(Debug, PartialEq)]
pub struct Post {
    title: String,
    body: String,
    scroll: u16,
}

#[derive(Debug, PartialEq)]
pub enum PostMessage {
    Post(String),
}

impl Post {
    pub fn create(name: String, ctx: &Context<TermApp>, map: &mut CursorMap) -> Self {
        let cp_name = name.clone();
        ctx.link().send_future(async move {
            let summaries =
                match reqwest::get(format!("http{HOST_ADDRESS}/api/v1/posts/{cp_name}")).await {
                    Ok(resp) => resp.json().await.unwrap_or_default(),
                    Err(e) => String::new(),
                };
            PostMessage::Post(summaries)
        });
        Self {
            title: name,
            body: String::new(),
            scroll: 0,
        }
    }

    pub fn handle_scroll(&mut self, dir: bool) {
        if dir {
            self.scroll = self.scroll.saturating_add(1);
        } else {
            self.scroll = self.scroll.saturating_sub(1);
        }
    }

    pub fn update(&mut self, msg: PostMessage, map: &mut CursorMap) {
        map.clear_after(1);
        match msg {
            PostMessage::Post(body) => {
                self.body = body;
            }
        }
    }

    pub fn draw(&self, mut rect: Rect, frame: &mut Frame) -> Rect {
        let widget = Paragraph::new(self.body.clone())
            .block(
                Block::new()
                    .borders(Borders::all())
                    .title(self.title.clone())
                    .title_alignment(Alignment::Center),
            )
            .scroll((self.scroll, 0));
        frame.render_widget(widget, rect);
        let mut state = SCROLL_STATE.lock().unwrap();
        state.insert(ScrollbarState::new(100));
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            rect,
            state.as_mut().unwrap(),
        );
        rect.y += rect.height;
        rect
    }
}
