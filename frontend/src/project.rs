use std::collections::{HashMap, HashSet};

use avid_rustacean_model::{Project, ProjectSummary};
use gloo_net::http::Request;
use ratatui::{prelude::*, widgets::*};
use webatui::{WebTermMessage, WebTerminal, backend::DehydratedSpan};
use yew::Context;
use yew_router::prelude::*;

use crate::{
    app::{AppBodyProps, ScrollMotion, TermApp},
    palette::{GruvboxColor, GruvboxExt},
    terminal::{NeedsHydration},
    utils::{padded_title, render_markdown, Markdown, MdLine, ScrollRef},
    Route,
};

#[derive(Debug, PartialEq, Clone)]
pub struct AllProjects {
    projects: Vec<(ProjectSummary, Vec<Line<'static>>)>,
    names: HashSet<String>,
    links: HashMap<String, String>,
}

#[derive(Debug)]
pub enum AllProjectsMessage {
    ProjectSummaries(Vec<ProjectSummary>),
    Clicked(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProjectView {
    title: String,
    body: Markdown,
    scroll: u16,
}

#[derive(Debug, PartialEq)]
pub enum ProjectMessage {
    Body(Project),
}

impl ProjectView {
    pub fn setup(&self, ctx: &Context<WebTerminal<TermApp>>) {
        let cp_title = self.title.clone();
        ctx.link().send_future(async move {
            let body = match Request::get(&format!("/api/v1/projects/{cp_title}"))
                .send()
                .await
            {
                Ok(resp) => resp.json().await.unwrap_or_default(),
                Err(_) => Project::default(),
            };
            WebTermMessage::new(ProjectMessage::Body(body))
        });
    }

    pub fn create(title: String) -> Self {
        Self {
            title,
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

    pub fn handle_scroll(&mut self, _dir: ScrollMotion) {}

    pub fn update(&mut self, _ctx: &Context<WebTerminal<TermApp>>, msg: ProjectMessage) {
        match msg {
            ProjectMessage::Body(body) => {
                self.body = Markdown::new(body.summary.name.clone(), body.body);
            }
        }
    }

    pub fn draw(&self, scroll: &ScrollRef, rect: Rect, frame: &mut Frame<'_>) {
        self.body.draw(scroll, rect, frame)
    }
}

impl AllProjects {
    pub fn setup(&self, ctx: &Context<WebTerminal<TermApp>>) {
        ctx.link().send_future(async move {
            let projects = match Request::get("/api/v1/projects").send().await {
                Ok(resp) => resp.json().await.unwrap_or_default(),
                Err(_) => Vec::new(),
            };
            WebTermMessage::new(AllProjectsMessage::ProjectSummaries(projects))
        });
    }

    pub fn create() -> Self {
        Self {
            projects: Vec::new(),
            names: HashSet::new(),
            links: HashMap::new(),
        }
    }

    pub fn hydrate(&self, ctx: &Context<WebTerminal<TermApp>>, span: &mut DehydratedSpan) {
        if let Some(link) = self.links.get(span.text()) {
            span.hyperlink(link.clone());
        } else if self.names.contains(span.text()) {
            let name = span.text().to_owned();
            span.on_click(
                ctx.link().callback(move |_| {
                    WebTermMessage::new(AllProjectsMessage::Clicked(name.clone()))
                }),
            );
        }
    }

    pub fn handle_scroll(&mut self, _dir: ScrollMotion) {}

    pub fn update(&mut self, ctx: &Context<WebTerminal<TermApp>>, msg: AllProjectsMessage) {
        match msg {
            AllProjectsMessage::ProjectSummaries(projects) => {
                self.projects = projects
                    .into_iter()
                    .map(|s| {
                        self.names.insert(s.name.clone());
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
            AllProjectsMessage::Clicked(name) => {
                let name = name.replace(' ', "-");
                ctx.link()
                    .send_message(WebTermMessage::new(AppBodyProps::Project(name.clone())));
                ctx.link()
                    .navigator()
                    .unwrap()
                    .push(&Route::Project { name });
            }
        }
    }

    pub fn draw(&self, scroll: &ScrollRef, rect: Rect, frame: &mut Frame<'_>) {
        let width = rect.width.saturating_sub(6) as usize;
        let mut lines = Vec::with_capacity(5 * self.projects.len() + 1);
        lines.push(
            Line::styled(
                "─".repeat(width),
                GruvboxColor::default_style().to_hydrate(),
            )
            .alignment(Alignment::Center),
        );
        for (summary, md) in &self.projects {
            lines.push(
                Line::styled(
                    summary.name.clone(),
                    GruvboxColor::teal()
                        .fg_style()
                        .to_hydrate()
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                )
                .alignment(Alignment::Center),
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
                        "Projects".into(),
                        GruvboxColor::green().full_style(GruvboxColor::dark_4()),
                    ))
                    .borders(Borders::ALL),
            );
        scroll.set_content_length(widget.line_count(rect.width.saturating_sub(2)));
        frame.render_widget(widget, rect);
    }
}
