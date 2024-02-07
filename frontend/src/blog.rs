use std::collections::{HashMap, HashSet};

use avid_rustacean_model::PostSummary;
use gloo_net::http::Request;
use ratatui::{prelude::*, widgets::*};
use webatui::prelude::*;
use yew::Context;
use yew_router::prelude::*;

use crate::{
    app::{AppBodyProps, ScrollMotion, TermApp},
    palette::{GruvboxColor, GruvboxExt},
    utils::{padded_title, render_markdown, MdLine, ScrollRef},
    Route,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Blog {
    summaries: Vec<(PostSummary, Vec<Line<'static>>)>,
    titles: HashSet<String>,
    links: HashMap<String, String>,
    scroll: u16,
}

#[derive(Debug)]
pub enum BlogMessage {
    PostSummaries(Vec<PostSummary>),
    Clicked(String),
}

impl Blog {
    pub fn setup(&self, ctx: &Context<WebTerminal<TermApp>>) {
        ctx.link().send_future(async move {
            let summaries = match Request::get("/api/v1/posts").send().await {
                Ok(resp) => resp.json().await.unwrap_or_default(),
                Err(_) => Vec::new(),
            };
            WebTermMessage::new(BlogMessage::PostSummaries(summaries))
        });
    }

    pub fn create() -> Self {
        Self {
            scroll: 0,
            summaries: Vec::new(),
            links: HashMap::new(),
            titles: HashSet::new(),
        }
    }

    pub fn hydrate(&self, ctx: &Context<WebTerminal<TermApp>>, span: &mut DehydratedSpan) {
        if let Some(link) = self.links.get(span.text()) {
            span.hyperlink(link.clone())
        } else if self.titles.contains(span.text()) {
            let title = span.text().to_owned();
            span.on_click(
                ctx.link()
                    .callback(move |_| WebTermMessage::new(BlogMessage::Clicked(title.clone()))),
            );
        }
    }

    pub fn handle_scroll(&mut self, dir: ScrollMotion) {
        match dir {
            ScrollMotion::Up => self.scroll = self.scroll.saturating_add(1),
            ScrollMotion::Down => self.scroll = self.scroll.saturating_sub(1),
        }
    }

    pub fn update(&mut self, ctx: TermContext<'_, TermApp>, msg: BlogMessage) {
        match msg {
            BlogMessage::PostSummaries(summaries) => {
                self.summaries = summaries
                    .into_iter()
                    .rev()
                    .map(|s| {
                        self.titles.insert(s.title.clone());
                        let lines = render_markdown(s.summary.clone(), &mut self.links)
                            .into_iter()
                            .filter_map(|l| match l {
                                MdLine::Plain(l) => Some(l.alignment(Alignment::Left)),
                                MdLine::Code(_) => None,
                            });
                        (s, lines.collect())
                    })
                    .collect();
            }
            BlogMessage::Clicked(name) => {
                let name = name.replace(' ', "-");
                ctx.ctx().link()
                    .send_message(WebTermMessage::new(AppBodyProps::Post(name.clone())));
                ctx.ctx().link().navigator().unwrap().push(&Route::Post { name });
            }
        }
    }

    pub fn draw(&self, scroll: &ScrollRef, rect: Rect, frame: &mut Frame<'_>) {
        let width = rect.width.saturating_sub(6) as usize;
        let mut lines = Vec::with_capacity(5 * self.summaries.len() + 1);
        lines.push(
            Line::styled(
                "─".repeat(width),
                GruvboxColor::default_style().to_hydrate(),
            )
            .alignment(Alignment::Center),
        );
        for (summary, md) in &self.summaries {
            lines.push(
                Line::styled(
                    summary.title.clone(),
                    GruvboxColor::teal()
                        .fg_style()
                        .to_hydrate()
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                )
                .alignment(Alignment::Center),
            );
            lines.push(
                Line::raw(format!(
                    "Published on: {}",
                    summary.create_on.format("%m/%d/%Y")
                ))
                .alignment(Alignment::Right),
            );
            lines.extend(md.iter().cloned());
            lines.push(Line::raw("═".repeat(width)).alignment(Alignment::Center));
        }
        lines.pop();
        lines.push(
            Line::styled(
                "─".repeat(width),
                GruvboxColor::default_style().to_hydrate(),
            )
            .alignment(Alignment::Center),
        );
        let widget = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::new()
                    .title(padded_title(
                        "Blog".into(),
                        GruvboxColor::green().full_style(GruvboxColor::dark_4()),
                    ))
                    .borders(Borders::ALL),
            );
        scroll.set_content_length(widget.line_count(rect.width.saturating_sub(2)));
        frame.render_widget(widget, rect);
    }
}
