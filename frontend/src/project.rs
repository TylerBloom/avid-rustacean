use std::{
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use avid_rustacean_model::{Project, ProjectSummary};
use ratatui::{prelude::*, widgets::*};
use yew::Context;
use yew_router::prelude::*;

use crate::{
    app::{AppBodyProps, Motion, TermApp, TermAppMsg},
    console_debug, console_log,
    palette::{GruvboxColor, GruvboxExt},
    terminal::{DehydratedSpan, NeedsHydration},
    utils::{render_markdown, Markdown, MdLine, ScrollRef},
    Route, HOST_ADDRESS,
};

#[derive(Debug, PartialEq)]
pub struct AllProjects {
    projects: Vec<(ProjectSummary, Vec<Line<'static>>)>,
    names: HashSet<String>,
    links: HashMap<String, String>,
}

#[derive(Debug)]
pub enum AllProjectsMessage {
    ProjectSummaries(Vec<ProjectSummary>),
    Clicked(String),
    Scrolled(bool),
}

#[derive(Debug, PartialEq)]
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
    pub fn create(title: String, ctx: &Context<TermApp>) -> Self {
        let cp_title = title.clone();
        ctx.link().send_future(async move {
            let body = match reqwest::get(format!("http{HOST_ADDRESS}/api/v1/projects/{cp_title}"))
                .await
            {
                Ok(resp) => resp.json().await.unwrap_or_default(),
                Err(_) => Project::default(),
            };
            ProjectMessage::Body(body)
        });
        Self {
            title,
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

    pub fn handle_scroll(&mut self, _dir: bool) {}

    pub fn update(&mut self, ctx: &Context<TermApp>, msg: ProjectMessage) {
        match msg {
            ProjectMessage::Body(body) => {
                self.body = Markdown::new(body.summary.name.clone(), body.body);
            }
        }
    }

    pub fn draw(&self, scroll: &ScrollRef, mut rect: Rect, frame: &mut Frame) {
        self.body.draw(scroll, rect, frame)
    }
}

impl AllProjects {
    pub fn create(ctx: &Context<TermApp>) -> Self {
        ctx.link().send_future(async move {
            let projects = match reqwest::get(format!("http{HOST_ADDRESS}/api/v1/projects")).await {
                Ok(resp) => resp.json().await.unwrap_or_default(),
                Err(e) => Vec::new(),
            };
            AllProjectsMessage::ProjectSummaries(projects)
        });
        Self {
            projects: Vec::new(),
            names: HashSet::new(),
            links: HashMap::new(),
        }
    }

    pub fn hydrate(&self, ctx: &Context<TermApp>, span: &mut DehydratedSpan) {
        if let Some(link) = self.links.get(span.text()) {
            span.hyperlink(link.clone());
        } else if self.names.contains(span.text()) {
            let name = span.text().to_owned();
            span.on_click(
                ctx.link()
                    .callback(move |_| AllProjectsMessage::Clicked(name.clone())),
            );
        }
    }

    pub fn handle_scroll(&mut self, dir: bool) {}

    pub fn update(&mut self, ctx: &Context<TermApp>, msg: AllProjectsMessage) {
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
            AllProjectsMessage::Scrolled(b) => self.handle_scroll(b),
            AllProjectsMessage::Clicked(name) => {
                ctx.link().send_message(AppBodyProps::Project(name.clone()));
                ctx.link()
                    .navigator()
                    .unwrap()
                    .push(&Route::Project { name });
            }
        }
    }

    pub fn draw(&self, view_start: usize, mut rect: Rect, frame: &mut Frame) {
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
            .block(Block::new().borders(Borders::all()));
        frame.render_widget(widget, rect);
    }
}
