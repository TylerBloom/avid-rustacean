use std::sync::Mutex;

use ratatui::{prelude::*, widgets::*};
use yew::Context;
use yew_router::prelude::*;

use crate::{
    app::{AppBodyProps, CursorMap, Motion, TermApp, TermAppMsg},
    console_log,
    palette::{GruvboxColor, GruvboxExt},
    terminal::{DehydratedSpan, NeedsHydration},
    Route, HOST_ADDRESS, utils::ScrollRef,
};

#[derive(Debug, PartialEq)]
pub struct Blog {
    summaries: Vec<(String, String, bool)>,
    scroll: u16,
}

#[derive(Debug)]
pub enum BlogMessage {
    PostSummaries(Vec<(String, String)>),
    Clicked(String),
}

impl Blog {
    pub fn create(ctx: &Context<TermApp>, map: &mut CursorMap) -> Self {
        ctx.link().send_future(async move {
            let summaries = match reqwest::get(format!("http{HOST_ADDRESS}/api/v1/posts")).await {
                Ok(resp) => resp.json().await.unwrap_or_default(),
                Err(e) => Vec::new(),
            };
            BlogMessage::PostSummaries(summaries)
        });
        Self {
            scroll: 0,
            summaries: Vec::new(),
        }
    }

    pub fn selected(&self) -> Option<usize> {
        None
    }

    pub fn hydrate(&self, ctx: &Context<TermApp>, span: &mut DehydratedSpan) {
        for (name, _, _) in self.summaries.iter() {
            if span.text() == name {
                let name = name.clone();
                span.on_click(
                    ctx.link()
                        .callback(move |_| BlogMessage::Clicked(name.clone())),
                );
            }
        }
    }

    pub fn handle_scroll(&mut self, dir: bool) {
        if dir {
            self.scroll = self.scroll.saturating_add(1);
        } else {
            self.scroll = self.scroll.saturating_sub(1);
        }
    }

    pub fn update(&mut self, ctx: &Context<TermApp>, scroll: &ScrollRef, msg: BlogMessage, map: &mut CursorMap) {
        match msg {
            BlogMessage::PostSummaries(summaries) => {
                map.clear_after(1);
                self.summaries = summaries
                    .into_iter()
                    .map(|(t, s)| {
                        map.append_and_push(t.clone());
                        (t, s, false)
                    })
                    .collect();
            }
            BlogMessage::Clicked(name) => {
                ctx.link().send_message(AppBodyProps::Post(name.clone()));
                ctx.link().navigator().unwrap().push(&Route::Post { name });
            }
        }
    }

    pub fn handle_motion(&mut self, motion: Motion, map: &CursorMap) {
        match map.get_position() {
            (0, y) if y > 0 && y <= self.summaries.len() => {
                self.summaries
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, (_, _, sel))| *sel = i + 1 == y);
            }
            _ => {
                self.summaries
                    .iter_mut()
                    .for_each(|(_, _, sel)| *sel = false);
            }
        }
    }

    pub fn draw(&self, mut rect: Rect, frame: &mut Frame) {
        let widget = Paragraph::new(
            self.summaries
                .iter()
                .map(|(t, _, sel)| get_line(t, *sel))
                .collect::<Vec<_>>(),
        )
        .alignment(Alignment::Center)
        .block(Block::new().borders(Borders::all()));
        frame.render_widget(widget, rect);
    }
}

fn get_line(s: &str, selected: bool) -> Line {
    let style = if selected {
        GruvboxColor::green().full_style(GruvboxColor::dark_3())
    } else {
        GruvboxColor::default_style()
    };
    Line::styled(s, style.to_hydrate())
}
