use std::collections::HashMap;

use avid_rustacean_model::{GruvboxColor, HomePage};
use gloo_net::http::Request;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};
use webatui::{WebTermMessage, WebTerminal, backend::DehydratedSpan};
use yew::Context;

use crate::{
    app::{ComponentMsg, ScrollMotion, TermApp},
    palette::GruvboxExt,
    utils::{padded_title, render_markdown, MdLine, ScrollRef},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Home {
    data: Paragraph<'static>,
    links: HashMap<String, String>,
    scroll: u16,
}

#[derive(Debug, PartialEq)]
pub enum HomeMessage {
    Data(HomePage),
}

impl Home {
    pub fn setup(&self, ctx: &Context<WebTerminal<TermApp>>) {
        ctx.link().send_future(async move {
            let home = match Request::get("/api/v1/home").send().await {
                Ok(resp) => resp.json().await.unwrap_or_default(),
                Err(_) => HomePage::default(),
            };
            WebTermMessage::new(ComponentMsg::Home(HomeMessage::Data(home)))
        });
    }

    pub fn create() -> Self {
        Self {
            data: Paragraph::default(),
            links: HashMap::new(),
            scroll: 0,
        }
    }

    pub fn handle_scroll(&mut self, dir: ScrollMotion) {
        match dir {
            ScrollMotion::Up => self.scroll = self.scroll.saturating_add(1),
            ScrollMotion::Down => self.scroll = self.scroll.saturating_sub(1),
        }
    }

    pub fn hydrate(&self, _ctx: &Context<WebTerminal<TermApp>>, _span: &mut DehydratedSpan) {}

    pub fn update(&mut self, msg: HomeMessage) {
        match msg {
            HomeMessage::Data(data) => {
                let lines: Vec<_> = render_markdown(data.body, &mut self.links)
                    .into_iter()
                    .filter_map(|l| match l {
                        MdLine::Plain(l) => Some(l),
                        MdLine::Code(_) => None,
                    })
                    .collect();
                self.data = Paragraph::new(lines)
                    .wrap(Wrap { trim: true })
                    .block(
                        Block::new()
                            .title(padded_title(
                                "Home".into(),
                                GruvboxColor::green().full_style(GruvboxColor::dark_4()),
                            ))
                            .borders(Borders::ALL)
                            .padding(Padding::horizontal(10)),
                    )
                    .alignment(Alignment::Center);
            }
        }
    }

    pub fn draw(&self, scroll: &ScrollRef, rect: Rect, frame: &mut Frame<'_>) {
        scroll.set_content_length(self.data.line_count(rect.width.saturating_sub(2)));
        frame.render_widget(
            self.data.clone().scroll((scroll.view_start() as u16, 0)),
            rect,
        );
    }
}
