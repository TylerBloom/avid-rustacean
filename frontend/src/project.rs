use std::{sync::Mutex, cell::RefCell, borrow::BorrowMut};

use ratatui::{prelude::*, widgets::*};
use yew::Context;

use crate::{
    app::{CursorMap, Motion, TermApp},
    console_log,
    palette::GruvboxColor,
    HOST_ADDRESS,
};

static ALL_SCROLL_STATE: Mutex<Option<ScrollbarState>> = Mutex::new(None);

#[derive(Debug, PartialEq)]
pub struct AllProjects {
    projects: Vec<(String, String, bool)>,
    scroll: u16,
    state: RefCell<ScrollbarState>,
}

#[derive(Debug)]
pub enum AllProjectsMessage {
    ProjectSummaries(Vec<(String, String)>),
    Scrolled(bool),
}

#[derive(Debug, PartialEq)]
pub struct Project {
    name: String,
    body: String,
    scroll: u16,
    line_count: u16,
    state: RefCell<ScrollbarState>,
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
            scroll: 0,
            // TODO: This should be calculated from the text... somehow
            line_count: 100,
            state: RefCell::new(ScrollbarState::new(100)),
        }
    }

    pub fn handle_scroll(&mut self, dir: bool) {
        if dir {
            self.scroll = std::cmp::min(self.line_count, self.scroll.saturating_add(1));
        } else {
            self.scroll = self.scroll.saturating_sub(1);
        }
        let state = self.state.borrow_mut().position(self.scroll as usize);
        *self.state.borrow_mut() = state;
    }

    pub fn update(&mut self, msg: ProjectMessage, map: &mut CursorMap) {
        match msg {
            ProjectMessage::Summary(body) => {
                map.clear_after(1);
                self.body = body;
            }
        }
    }

    pub fn draw(&self, mut rect: Rect, frame: &mut Frame) -> Rect {
        console_log(format!("The project data: {self:?}"));
        let widget = Paragraph::new(self.body.clone())
            .block(
                Block::new()
                    .borders(Borders::all())
                    .title(self.name.clone())
                    .title_alignment(Alignment::Center),
            )
            .scroll((self.scroll, 0));
        frame.render_widget(widget, rect);
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            rect,
            &mut self.state.borrow_mut(),
        );
        rect.y += rect.height;
        rect
    }
}

impl AllProjects {
    pub fn create(ctx: &Context<TermApp>, map: &mut CursorMap) -> Self {
        ALL_SCROLL_STATE
            .lock()
            .unwrap()
            .insert(ScrollbarState::default());
        ctx.link().send_future(async move {
            let projects = match reqwest::get(format!("http{HOST_ADDRESS}/api/v1/projects")).await {
                Ok(resp) => resp.json().await.unwrap_or_default(),
                Err(e) => Vec::new(),
            };
            AllProjectsMessage::ProjectSummaries(projects)
        });
        Self {
            projects: Vec::new(),
            scroll: 0,
            state: RefCell::new(ScrollbarState::new(0)),
        }
    }

    pub fn handle_scroll(&mut self, dir: bool) {
        if dir {
            self.scroll = std::cmp::min(self.projects.len() as u16, self.scroll.saturating_add(1));
        } else {
            self.scroll = self.scroll.saturating_sub(1);
        }
        let state = self.state.borrow_mut().position(self.scroll as usize);
        *self.state.borrow_mut() = state;
    }

    pub fn update(&mut self, msg: AllProjectsMessage, map: &mut CursorMap) {
        match msg {
            AllProjectsMessage::ProjectSummaries(projects) => {
                self.projects = projects.into_iter().map(|(n, s)| (n, s, false)).collect();
                let state = self.state.borrow_mut().content_length(self.projects.len());
                *self.state.borrow_mut() = state;
                map.clear_after(1);
                for (title, _, _) in self.projects.iter() {
                    map.append_and_push(title.clone());
                }
            }
            AllProjectsMessage::Scrolled(b) => self.handle_scroll(b),
        }
    }

    pub fn handle_motion(&mut self, motion: Motion, map: &CursorMap) {
        match map.get_position() {
            (0, y) if y > 0 && y <= self.projects.len() => {
                if y as u16 > self.scroll {
                    self.scroll = y as u16;
                    let state = self.state.borrow_mut().position(y);
                    *self.state.borrow_mut() = state;
                }
                self.projects
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, (_, _, sel))| *sel = i + 1 == y);
            }
            _ => {
                self.projects
                    .iter_mut()
                    .for_each(|(_, _, sel)| *sel = false);
            }
        }
    }

    pub fn draw(&self, mut rect: Rect, frame: &mut Frame) -> Rect {
        let widget = Paragraph::new(
            self.projects
                .iter()
                .map(|(s, _, sel)| get_line(s, *sel))
                .collect::<Vec<_>>(),
        )
        .block(Block::new().borders(Borders::all()))
        .scroll((self.scroll, 0));
        frame.render_widget(widget, rect);
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            rect,
            &mut self.state.borrow_mut(),
        );
        rect.y += rect.height;
        rect
    }
}

fn get_line(s: &str, selected: bool) -> Line {
    let style = if selected {
        GruvboxColor::green().full_style(GruvboxColor::dark_3())
    } else {
        GruvboxColor::default_style()
    };
    Line::styled(s, style)
}
