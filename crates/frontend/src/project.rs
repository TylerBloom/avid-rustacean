use std::collections::{HashMap};

use gloo_net::http::Request;
use ratatui::{prelude::*, widgets::*};
use webatui::prelude::*;
use yew::Context;

use crate::{
    app::{TermApp},
    palette::{GruvboxColor, GruvboxExt},
    utils::{padded_title, render_markdown, MdLine, ScrollRef},
};

#[derive(Debug, PartialEq, Clone)]
pub struct AllProjects {
    projects: Vec<Line<'static>>,
    links: HashMap<String, String>,
}

#[derive(Debug)]
pub enum AllProjectsMessage {
    ProjectSummaries(avid_rustacean_model::Markdown),
}

impl AllProjects {
    pub fn setup(&self, ctx: &Context<WebTerminal<TermApp>>) {
        ctx.link().send_future(async move {
            let projects = match Request::get("/tui/projects.json").send().await {
                Ok(resp) => resp.json().await.unwrap_or_default(),
                Err(_) => Default::default(),
            };
            web_sys::console::log_1(&format!("{projects:?}").into());
            WebTermMessage::new(AllProjectsMessage::ProjectSummaries(projects))
        });
    }

    pub fn create() -> Self {
        Self {
            projects: Vec::new(),
            links: HashMap::new(),
        }
    }

    pub fn hydrate(&self, _ctx: &Context<WebTerminal<TermApp>>, span: &mut DehydratedSpan) {
        if let Some(link) = self.links.get(span.text()) {
            span.hyperlink(link.clone());
        }
    }

    pub fn handle_scroll(&mut self, _dir: ScrollMotion) {}

    pub fn update(&mut self, _ctx: TermContext<'_, TermApp>, msg: AllProjectsMessage) {
        match msg {
            AllProjectsMessage::ProjectSummaries(projects) => {
                let projects = render_markdown(projects, &mut self.links)
                    .into_iter()
                    .filter_map(|l| match l {
                        MdLine::Plain(l) => Some(l.alignment(Alignment::Center)),
                        MdLine::Code(_) => None,
                    })
                    .collect();
                self.projects = projects;
            }
        }
    }

    pub fn draw(&self, scroll: &ScrollRef, rect: Rect, frame: &mut Frame<'_>) {
        let lines = self.projects.clone();
        let widget = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::new()
                    .title(padded_title(
                        "Projects".into(),
                        GruvboxColor::green().full_style(GruvboxColor::dark_4()),
                    ))
                    .borders(Borders::ALL),
            );
        scroll.set_content_length(widget.line_count(rect.width.saturating_sub(2)));
        frame.render_widget(widget, rect);
    }
}
