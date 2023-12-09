use std::{
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
    sync::Mutex,
};

use ratatui::{prelude::*, widgets::*};
use yew::Context;
use yew_router::prelude::*;

use crate::{
    app::{AppBodyProps, CursorMap, Motion, TermApp, TermAppMsg},
    console_debug, console_log,
    palette::{GruvboxColor, GruvboxExt},
    terminal::{DehydratedSpan, NeedsHydration},
    Route, HOST_ADDRESS, utils::ScrollRef,
};

#[derive(Debug, PartialEq)]
pub struct AllProjects {
    projects: Vec<(String, String)>,
    selected: Option<usize>,
}

#[derive(Debug)]
pub enum AllProjectsMessage {
    ProjectSummaries(Vec<(String, String)>),
    Clicked(String),
    Scrolled(bool),
}

#[derive(Debug, PartialEq)]
pub struct Project {
    name: String,
    body: String,
    line_count: u16,
}

#[derive(Debug, PartialEq)]
pub enum ProjectMessage {
    Summary(String),
}

impl Project {
    pub fn create(name: String, ctx: &Context<TermApp>, map: &mut CursorMap) -> Self {
        let cp_name = name.clone();
        ctx.link().send_future(async move {
            let body =
                match reqwest::get(format!("http{HOST_ADDRESS}/api/v1/projects/{cp_name}")).await {
                    Ok(resp) => resp.json().await.unwrap_or_default(),
                    Err(_) => String::new(),
                };
            ProjectMessage::Summary(body)
        });
        Self {
            body: String::new(),
            name,
            // TODO: This should be calculated from the text... somehow
            line_count: 100,
        }
    }

    pub fn selected(&self) -> Option<usize> {
        None
    }

    pub fn hydrate(&self, ctx: &Context<TermApp>, span: &mut DehydratedSpan) {
        // TODO: Hydrate as needed
    }

    pub fn handle_scroll(&mut self, _dir: bool) {}

    pub fn update(
        &mut self,
        ctx: &Context<TermApp>,
        scroll: &ScrollRef,
        msg: ProjectMessage,
        map: &mut CursorMap,
    ) {
        match msg {
            ProjectMessage::Summary(body) => {
                scroll.set_content_length(body.len());
                map.clear_after(1);
                self.body = body;
            }
        }
    }

    pub fn draw(&self, view_start: usize, mut rect: Rect, frame: &mut Frame) {
        let view_start = view_start as u16;
        let widget = Paragraph::new(self.body.clone())
            .block(
                Block::new()
                    .borders(Borders::all())
                    .title(self.name.clone())
                    .title_alignment(Alignment::Center),
            )
            .scroll((view_start, 0));
        frame.render_widget(widget, rect);
    }
}

impl AllProjects {
    pub fn create(ctx: &Context<TermApp>, map: &mut CursorMap) -> Self {
        ctx.link().send_future(async move {
            let projects = match reqwest::get(format!("http{HOST_ADDRESS}/api/v1/projects")).await {
                Ok(resp) => resp.json().await.unwrap_or_default(),
                Err(e) => Vec::new(),
            };
            AllProjectsMessage::ProjectSummaries(projects)
        });
        Self {
            projects: Vec::new(),
            selected: None,
        }
    }

    pub fn selected(&self) -> Option<usize> {
        None
    }

    pub fn hydrate(&self, ctx: &Context<TermApp>, span: &mut DehydratedSpan) {
        for (name, _) in self.projects.iter() {
            if span.text() == name {
                let name = name.clone();
                span.on_click(
                    ctx.link()
                        .callback(move |_| AllProjectsMessage::Clicked(name.clone())),
                );
            }
        }
    }

    pub fn handle_scroll(&mut self, dir: bool) {
    }

    pub fn update(
        &mut self,
        ctx: &Context<TermApp>,
        scroll: &ScrollRef,
        msg: AllProjectsMessage,
        map: &mut CursorMap,
    ) {
        match msg {
            AllProjectsMessage::ProjectSummaries(projects) => {
                scroll.set_content_length(projects.len());
                self.projects = projects;
                map.clear_after(1);
                for (title, _) in self.projects.iter() {
                    map.append_and_push(title.clone());
                }
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

    pub fn handle_motion(&mut self, motion: Motion, map: &CursorMap) {
        match map.get_position() {
            (0, y) if y > 0 && y <= self.projects.len() => {
                self.selected.insert(y - 1);
            }
            _ => {
                let _ = self.selected.take();
            }
        }
    }

    pub fn draw(&self, view_start: usize, mut rect: Rect, frame: &mut Frame) {
        let widget = Paragraph::new(
            self.projects
                .iter()
                .enumerate()
                .map(|(i, (s, _))| get_line(s, self.selected.map(|s| s == i).unwrap_or_default()))
                .collect::<Vec<_>>(),
        )
        .block(Block::new().borders(Borders::all()))
        .scroll((view_start as u16, 0));
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
