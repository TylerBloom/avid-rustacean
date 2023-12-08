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
    Route, HOST_ADDRESS,
};

#[derive(Debug, PartialEq)]
pub struct AllProjects {
    projects: Vec<(String, String)>,
    selected: Option<usize>,
    state: ScrollRef,
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
    state: ScrollRef,
}

#[derive(Debug, PartialEq)]
pub enum ProjectMessage {
    Summary(String),
}

/// A container for managing the logic for a well-formated scroll bar.
#[derive(Debug, PartialEq)]
pub struct ScrollRef {
    /// The number of lines that could be displayed.
    content_length: RefCell<usize>,
    /// The last-known length of the viewport. Used to calculate the position of the bottom-most
    /// element.
    view_length: RefCell<usize>,
    /// The line number of where the viewport starts.
    view_start: RefCell<usize>,
    /// The scrollbar state needed to render a scrollbar.
    state: RefCell<ScrollbarState>,
}

impl ScrollRef {
    fn new(content_length: usize, lines: usize) -> Self {
        Self {
            content_length: RefCell::new(content_length),
            view_length: RefCell::new(0),
            view_start: RefCell::new(0),
            state: RefCell::new(ScrollbarState::new(lines)),
        }
    }

    /// Renders the scrollbar into a frame
    fn render_scroll(&self, frame: &mut Frame, bar: Scrollbar, rect: Rect) {
        self.set_view_length(rect.height as usize);
        frame.render_stateful_widget(bar, rect, &mut self.state.borrow_mut());
    }

    /// Sets the number of lines of content to be displayed
    fn set_content_length(&self, lines: usize) {
        *self.content_length.borrow_mut() = lines;
        let state = self.state.borrow_mut().content_length(lines);
        *self.state.borrow_mut() = state;
    }

    fn content_length(&self) -> usize {
        *self.content_length.borrow()
    }

    /// Sets the length in the scrollbar state.
    fn set_view_length(&self, lines: usize) {
        *self.view_length.borrow_mut() = lines;
        let start = std::cmp::min(
            *self.view_start.borrow(),
            (*self.content_length.borrow()).saturating_sub(lines - 1),
        );
        *self.view_start.borrow_mut() = start;
        let inner_content_length =
            (*self.content_length.borrow()).saturating_sub(*self.view_length.borrow());
        let state = self.state.borrow().content_length(inner_content_length);
        *self.state.borrow_mut() = state;
    }

    /// Gets the scroll index.
    fn view_start(&self) -> usize {
        *self.view_start.borrow()
    }

    /// Gets the length of the view port.
    fn view_length(&self) -> usize {
        *self.view_length.borrow()
    }

    fn set_view_start(&self, view_start: usize) {
        *self.view_start.borrow_mut() = view_start;
        let state = self.state.borrow().position(view_start);
        *self.state.borrow_mut() = state;
    }

    /// Moves the scroll state down.
    fn next(&mut self) {
        *self.view_start.get_mut() = self
            .view_start
            .get_mut()
            .checked_add(1)
            .unwrap_or(*self.content_length.borrow());
        self.state.get_mut().next()
    }

    /// Moves the scroll state up.
    fn prev(&mut self) {
        *self.view_start.get_mut() = self.view_start.get_mut().saturating_sub(1);
        self.state.get_mut().prev()
    }
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
            state: ScrollRef::new(0, 100),
        }
    }

    pub fn hydrate(&self, ctx: &Context<TermApp>, span: &mut DehydratedSpan) {
        // TODO: Hydrate as needed
    }

    pub fn handle_scroll(&mut self, dir: bool) {
        if dir {
            self.state.next()
        } else {
            self.state.prev()
        }
    }

    pub fn update(&mut self, ctx: &Context<TermApp>, msg: ProjectMessage, map: &mut CursorMap) {
        match msg {
            ProjectMessage::Summary(body) => {
                self.state.set_content_length(body.len());
                map.clear_after(1);
                self.body = body;
            }
        }
    }

    pub fn draw(&self, mut rect: Rect, frame: &mut Frame) -> Rect {
        console_log(format!("The project data: {self:?}"));
        self.state.set_view_length(rect.height as usize);
        let view_start = self.state.view_start() as u16;
        let widget = Paragraph::new(self.body.clone())
            .block(
                Block::new()
                    .borders(Borders::all())
                    .title(self.name.clone())
                    .title_alignment(Alignment::Center),
            )
            .scroll((view_start, 0));
        frame.render_widget(widget, rect);
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            rect,
            &mut self.state.state.borrow_mut(),
        );
        rect.y += rect.height;
        rect
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
            state: ScrollRef::new(0, 0),
        }
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
        if dir {
            self.state.next()
        } else {
            self.state.prev()
        }
    }

    pub fn update(&mut self, ctx: &Context<TermApp>, msg: AllProjectsMessage, map: &mut CursorMap) {
        match msg {
            AllProjectsMessage::ProjectSummaries(projects) => {
                self.state.set_content_length(projects.len());
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

    pub fn draw(&self, mut rect: Rect, frame: &mut Frame) -> Rect {
        console_debug(self.selected);
        console_debug(&self.state);
        if let Some(sel) = self.selected {
            let view_start = self.state.view_start();
            if sel < view_start {
                self.state.set_view_start(sel);
            } else if sel > view_start + self.state.view_length().saturating_sub(3) {
                console_log("Selected is greater than start + length");
                let length = self.state.view_length();
                self.state
                    .set_view_start(sel.saturating_sub(length.saturating_sub(3)));
            }
        }
        let widget = Paragraph::new(
            self.projects
                .iter()
                .enumerate()
                .map(|(i, (s, _))| get_line(s, self.selected.map(|s| s == i).unwrap_or_default()))
                .collect::<Vec<_>>(),
        )
        .block(Block::new().borders(Borders::all()))
        .scroll((self.state.view_start() as u16, 0));
        frame.render_widget(widget, rect);
        self.state.render_scroll(
            frame,
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            rect,
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
    Line::styled(s, style.to_hydrate())
}
