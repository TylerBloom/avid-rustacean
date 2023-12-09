use std::{cmp::Ordering, collections::HashMap, ops::Range};

use yew::{Component, Context, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    blog::{Blog, BlogMessage},
    console_debug, console_log,
    home::Home,
    palette::{GruvboxColor, GruvboxExt},
    posts::{Post, PostMessage},
    project::{AllProjects, AllProjectsMessage, Project, ProjectMessage},
    terminal::{get_window_size, DehydratedSpan, NeedsHydration},
    utils::{ScrollRef, padded_title},
    Route, TERMINAL,
};
use derive_more::From;
use js_sys::Function;
use ratatui::{prelude::*, widgets::*};
use wasm_bindgen::{prelude::Closure, JsValue};
use yew::prelude::*;

/// This module contains all of the machinery to run the UI app. The UI app is a single page
/// application consisting of the header, body, and footer. The body is changed when switching
/// between tabs/"pages". The app holds all of the logic for interacting with the browser window,
/// including switching between tabs and tracking the cursor.

pub struct TermApp {
    /// The tracker for the cursor
    cursor_map: CursorMap,
    /// The body of the UI
    body: AppBody,
}

/// The body used for the app on construction.
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct TermAppProps {
    pub body: AppBodyProps,
}

pub struct AppBody {
    inner: AppBodyInner,
    // TODO: Rename to scroll
    scroll: ScrollRef,
}

/// The different main sections the user might find themselves in.
#[derive(Debug, PartialEq, From)]
enum AppBodyInner {
    Home(Home),
    AllProjects(AllProjects),
    Project(Project),
    Blog(Blog),
    Post(Post),
}

impl AppBody {
    fn new<I: Into<AppBodyInner>>(inner: I) -> Self {
        Self {
            inner: inner.into(),
            scroll: ScrollRef::new(0, 0),
        }
    }

    fn swap_inner<I: Into<AppBodyInner>>(&mut self, inner: I) {
        self.inner = inner.into();
        self.scroll.set_view_start(0);
    }

    fn draw(&self, chunk: Rect, frame: &mut Frame) {
        if let Some(sel) = self.inner.selected() {
            let view_start = self.scroll.view_start();
            if sel < view_start {
                self.scroll.set_view_start(sel);
            } else if sel > view_start + self.scroll.view_length().saturating_sub(3) {
                let length = self.scroll.view_length();
                self.scroll
                    .set_view_start(sel.saturating_sub(length.saturating_sub(3)));
            }
        }
        self.inner.draw(&self.scroll, chunk, frame);
        self.scroll.render_scroll(
            frame,
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            chunk,
        );
    }

    fn hydrate(&self, ctx: &Context<TermApp>, span: &mut DehydratedSpan) {
        self.inner.hydrate(ctx, span)
    }

    fn update(&mut self, ctx: &Context<TermApp>, msg: ComponentMsg, map: &mut CursorMap) {
        self.inner.update(ctx, &self.scroll, msg, map)
    }

    fn handle_scroll(&mut self, dir: bool) {
        self.inner.handle_scroll(dir);
        if dir {
            self.scroll.next()
        } else {
            self.scroll.prev()
        }
    }

    fn handle_motion(&mut self, motion: Motion, map: &CursorMap) {
        self.inner.handle_motion(motion, map)
    }

    fn handle_enter(&mut self, ctx: &Context<TermApp>, map: &CursorMap) {
        self.inner.handle_enter(ctx, map)
    }
}

impl AppBodyInner {
    fn draw(&self, scroll: &ScrollRef, chunk: Rect, frame: &mut Frame) {
        match self {
            Self::Home(home) => home.draw(chunk, frame),
            Self::AllProjects(projects) => projects.draw(scroll.view_start(), chunk, frame),
            Self::Project(proj) => proj.draw(scroll.view_start(), chunk, frame),
            Self::Blog(blog) => blog.draw(chunk, frame),
            Self::Post(post) => post.draw(scroll, chunk, frame),
        }
    }

    /// Returns the line number of the currently selected item.
    fn selected(&self) -> Option<usize> {
        match self {
            AppBodyInner::Home(home) => home.selected(),
            AppBodyInner::AllProjects(projects) => projects.selected(),
            AppBodyInner::Project(project) => project.selected(),
            AppBodyInner::Blog(blog) => blog.selected(),
            AppBodyInner::Post(post) => post.selected(),
        }
    }

    fn hydrate(&self, ctx: &Context<TermApp>, span: &mut DehydratedSpan) {
        match self {
            Self::Home(home) => home.hydrate(ctx, span),
            Self::AllProjects(projects) => projects.hydrate(ctx, span),
            Self::Project(proj) => proj.hydrate(ctx, span),
            Self::Blog(blog) => blog.hydrate(ctx, span),
            Self::Post(post) => post.hydrate(ctx, span),
        }
    }

    fn update(
        &mut self,
        ctx: &Context<TermApp>,
        scroll: &ScrollRef,
        msg: ComponentMsg,
        map: &mut CursorMap,
    ) {
        match (self, msg) {
            (Self::AllProjects(body), ComponentMsg::AllProjects(msg)) => {
                body.update(ctx, scroll, msg, map)
            }
            (Self::Project(body), ComponentMsg::Project(msg)) => body.update(ctx, scroll, msg, map),
            (Self::Blog(body), ComponentMsg::Blog(msg)) => body.update(ctx, scroll, msg, map),
            (Self::Post(body), ComponentMsg::Post(msg)) => body.update(ctx, msg, map),
            _ => unreachable!("How did you get here? Open a PR, please"),
        }
    }

    fn handle_scroll(&mut self, dir: bool) {
        match self {
            Self::Home(home) => {}
            Self::AllProjects(projects) => projects.handle_scroll(dir),
            Self::Blog(blog) => blog.handle_scroll(dir),
            Self::Project(proj) => proj.handle_scroll(dir),
            Self::Post(post) => post.handle_scroll(dir),
        }
    }

    fn handle_motion(&mut self, motion: Motion, map: &CursorMap) {
        match self {
            Self::AllProjects(projects) => projects.handle_motion(motion, map),
            Self::Blog(blog) => blog.handle_motion(motion, map),
            Self::Home(home) => {}
            Self::Project(proj) => {}
            Self::Post(post) => {}
        }
    }

    fn handle_enter(&mut self, ctx: &Context<TermApp>, map: &CursorMap) {
        match self {
            Self::AllProjects(_) => {
                let nav = ctx.link().navigator().unwrap();
                let name = map.get_hovering().to_owned();
                nav.push(&Route::Project { name });
            }
            Self::Blog(_) => {
                let nav = ctx.link().navigator().unwrap();
                let name = map.get_hovering().to_owned();
                nav.push(&Route::Post { name });
            }
            Self::Home(_) => {}
            Self::Project(_) => {}
            Self::Post(_) => {}
        }
    }
}

/// The different main sections the user might find themselves in.
#[derive(Debug, PartialEq, Clone)]
pub enum AppBodyProps {
    Home,
    AllProjects,
    Project(String),
    Blog,
    Post(String),
}

impl AppBodyProps {
    fn create_body(self, ctx: &Context<TermApp>, map: &mut CursorMap) -> AppBody {
        let inner = match self {
            AppBodyProps::Home => AppBodyInner::Home(Home::create(map)),
            AppBodyProps::AllProjects => AppBodyInner::AllProjects(AllProjects::create(ctx, map)),
            AppBodyProps::Project(name) => AppBodyInner::Project(Project::create(name, ctx, map)),
            AppBodyProps::Blog => AppBodyInner::Blog(Blog::create(ctx, map)),
            AppBodyProps::Post(name) => AppBodyInner::Post(Post::create(name, ctx, map)),
        };
        AppBody::new(inner)
    }
}

#[derive(Debug, From)]
pub enum TermAppMsg {
    Resized,
    Entered,
    Clicked(AppBodyProps),
    SelectedMoved(Motion),
    // TODO: Replace bool with "up" or "down"
    // true = up, down = false
    Scrolled(bool),
    ComponentMsg(ComponentMsg),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Motion {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, From)]
pub enum ComponentMsg {
    AllProjects(AllProjectsMessage),
    Blog(BlogMessage),
    Project(ProjectMessage),
    Post(PostMessage),
}

impl TermApp {
    fn draw(&self, area: Rect, frame: &mut Frame) {
        let (width, height) = get_window_size();
        let chunks = Layout::new(
            Direction::Vertical,
            [
                Constraint::Min(3),
                Constraint::Percentage(100),
                Constraint::Min(3),
            ],
        )
        .split(area);
        self.draw_header(chunks[0], frame);
        self.body.draw(chunks[1], frame);
        self.draw_footer(chunks[2], frame);
    }

    fn draw_header(&self, rect: Rect, frame: &mut Frame) {
        let titles = vec![
            Line::styled("Home", GruvboxColor::default_style().to_hydrate()),
            Line::styled("Projects", GruvboxColor::default_style().to_hydrate()),
            Line::styled("Blog", GruvboxColor::default_style().to_hydrate()),
        ];
        let mut tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(padded_title("The Avid Rustacean".to_owned(), Style::new()))
                    .title_alignment(Alignment::Center),
            )
            .style(GruvboxColor::teal().full_style(GruvboxColor::dark_2()))
            .highlight_style(GruvboxColor::green().full_style(GruvboxColor::dark_3()));
        console_debug(self.cursor_map.get_position());
        let index = match self.cursor_map.get_position() {
            (x, 0) => x,
            _ => match &self.body.inner {
                AppBodyInner::Home(_) => 0,
                AppBodyInner::AllProjects(_) | AppBodyInner::Project(_) => 1,
                AppBodyInner::Blog(_) | AppBodyInner::Post(_) => 2,
            },
        };
        tabs = tabs.select(index);
        if let (x, 0) = self.cursor_map.get_position() {
            console_log("Selecting a tab...");
        }
        frame.render_widget(tabs, rect);
    }

    fn draw_footer(&self, rect: Rect, frame: &mut Frame) {
        let style = GruvboxColor::default_fg().full_style(GruvboxColor::default_bg());
        let widget = Paragraph::new("The footer")
            .block(Block::new().borders(Borders::ALL))
            .style(style)
            .alignment(Alignment::Center);
        frame.render_widget(widget, rect);
    }
}

impl Component for TermApp {
    type Message = TermAppMsg;
    type Properties = TermAppProps;

    fn create(ctx: &Context<Self>) -> Self {
        let window = web_sys::window().unwrap();
        // Bind a function to the "on-resize" window event
        let cb = ctx.link().callback(|()| TermAppMsg::Resized);
        let func = move || cb.emit(());
        let func: Function = Closure::<dyn 'static + Fn()>::new(func)
            .into_js_value()
            .into();
        window.set_onresize(Some(&func));
        // Bind a function to the "on-keypress" window event
        let cb = ctx.link().callback(|()| TermAppMsg::Entered);
        let func = move |event: JsValue| {
            let event: KeyboardEvent = event.into();
            if event.key() == "Enter" {
                cb.emit(())
            }
        };
        let func: Function = Closure::<dyn 'static + Fn(JsValue)>::new(func)
            .into_js_value()
            .into();
        window.set_onkeypress(Some(&func));
        // Bind a function to the "on-wheel" window event
        let cb = ctx.link().callback(|msg: TermAppMsg| msg);
        let func = move |event: JsValue| {
            let event: WheelEvent = event.into();
            match event.delta_y().partial_cmp(&0.0) {
                Some(Ordering::Less) => cb.emit(TermAppMsg::Scrolled(false)),
                Some(Ordering::Greater) => cb.emit(TermAppMsg::Scrolled(true)),
                _ => {}
            }
        };
        let func: Function = Closure::<dyn 'static + Fn(JsValue)>::new(func)
            .into_js_value()
            .into();
        window.set_onwheel(Some(&func));

        // Bind a function to the "on-keydown" window event
        let cb = ctx.link().callback(|msg: TermAppMsg| msg);
        let func = move |event: JsValue| {
            let event: KeyboardEvent = event.into();
            match event.key().as_str() {
                "ArrowUp" => cb.emit(TermAppMsg::SelectedMoved(Motion::Up)),
                "ArrowDown" => cb.emit(TermAppMsg::SelectedMoved(Motion::Down)),
                "ArrowLeft" => cb.emit(TermAppMsg::SelectedMoved(Motion::Left)),
                "ArrowRight" => cb.emit(TermAppMsg::SelectedMoved(Motion::Right)),
                _ => {}
            }
        };
        let func: Function = Closure::<dyn 'static + Fn(JsValue)>::new(func)
            .into_js_value()
            .into();
        window.set_onkeydown(Some(&func));
        // Create cursor map
        let mut cursor_map = CursorMap::new();
        cursor_map.push(String::from("home"));
        cursor_map.push(String::from("projects"));
        cursor_map.push(String::from("blog"));
        match ctx.props().body {
            AppBodyProps::AllProjects => {
                cursor_map.move_right();
            }
            AppBodyProps::Blog => {
                cursor_map.move_left();
            }
            AppBodyProps::Home | AppBodyProps::Project(_) | AppBodyProps::Post(_) => {}
        }

        // Create the viewer
        let body = ctx.props().body.clone().create_body(ctx, &mut cursor_map);
        Self { cursor_map, body }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        console_log(format!("Got message: {msg:?}"));
        match msg {
            TermAppMsg::Resized => TERMINAL.term().backend_mut().resize_buffer(),
            TermAppMsg::Entered => match self.cursor_map.get_position() {
                (0, 0) => {
                    ctx.link().navigator().unwrap().push(&Route::Home);
                    self.body.swap_inner(Home::create(&mut self.cursor_map));
                }
                (1, 0) => {
                    ctx.link().navigator().unwrap().push(&Route::AllProjects);
                    self.body
                        .swap_inner(AllProjects::create(ctx, &mut self.cursor_map));
                }
                (2, 0) => {
                    ctx.link().navigator().unwrap().push(&Route::Blog);
                    self.body
                        .swap_inner(Blog::create(ctx, &mut self.cursor_map));
                }
                _ => match &self.body.inner {
                    AppBodyInner::AllProjects(_) => {
                        let nav = ctx.link().navigator().unwrap();
                        let name = self.cursor_map.get_hovering().to_owned();
                        nav.push(&Route::Project { name: name.clone() });
                        self.cursor_map.clear_after(1);
                        self.body
                            .swap_inner(Project::create(name, ctx, &mut self.cursor_map));
                    }
                    AppBodyInner::Blog(_) => {
                        let nav = ctx.link().navigator().unwrap();
                        let name = self.cursor_map.get_hovering().to_owned();
                        nav.push(&Route::Post { name: name.clone() });
                        self.cursor_map.clear_after(1);
                        self.body
                            .swap_inner(Post::create(name, ctx, &mut self.cursor_map));
                    }
                    _ => {}
                },
            },
            TermAppMsg::ComponentMsg(msg) => self.body.update(ctx, msg, &mut self.cursor_map),
            TermAppMsg::SelectedMoved(motion) => {
                self.cursor_map.motion(motion);
                self.body.handle_motion(motion, &self.cursor_map);
            }
            TermAppMsg::Scrolled(b) => self.body.handle_scroll(b),
            TermAppMsg::Clicked(page) => {
                match &page {
                    AppBodyProps::Home => ctx.link().navigator().unwrap().push(&Route::Home),
                    AppBodyProps::AllProjects => {
                        ctx.link().navigator().unwrap().push(&Route::AllProjects)
                    }
                    AppBodyProps::Blog => ctx.link().navigator().unwrap().push(&Route::Blog),
                    AppBodyProps::Project(name) => {
                        ctx.link().navigator().unwrap().push(&Route::Project {
                            name: name.to_owned(),
                        });
                    }
                    AppBodyProps::Post(name) => {
                        ctx.link().navigator().unwrap().push(&Route::Post {
                            name: name.to_owned(),
                        });
                    }
                }
                self.body = page.create_body(ctx, &mut self.cursor_map);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut term = TERMINAL.term();
        let area = term.size().unwrap();
        term.draw(|frame| self.draw(area, frame)).unwrap();
        term.backend_mut().hydrate(|span| match span.text().trim() {
            "Home" => span.on_click(ctx.link().callback(|_| AppBodyProps::Home)),
            "Projects" => span.on_click(ctx.link().callback(|_| AppBodyProps::AllProjects)),
            "Blog" => span.on_click(ctx.link().callback(|_| AppBodyProps::Blog)),
            _ => self.body.hydrate(ctx, span),
        })
    }
}

/// The matrix of the relative directions the cursor can travel.
pub struct CursorMap {
    map: Vec<Vec<String>>,
    cursor: (usize, usize),
}

impl CursorMap {
    fn new() -> Self {
        Self {
            map: vec![Vec::new()],
            cursor: (0, 0),
        }
    }

    /// Adds a string to the end of the current line.
    pub fn push(&mut self, next: String) {
        self.map.last_mut().unwrap().push(next)
    }

    /// Adds a new, blank line onto which this item and all subsequent item will be pushed.
    pub fn append_and_push(&mut self, next: String) {
        self.map.push(Vec::new());
        self.push(next)
    }

    /// Returns the current position of the cursor
    pub fn get_position(&self) -> (usize, usize) {
        self.cursor
    }

    /// Gets the value that the cursor is hovering over.
    pub fn get_hovering(&self) -> &str {
        &self.map[self.cursor.1][self.cursor.0]
    }

    /// Clears some number of rows from the map
    pub fn clear_after(&mut self, index: usize) {
        self.map.drain(index..);
        if self.cursor.1 >= self.map.len() {
            self.cursor.1 = self.map.len() - 1;
        }

        if self.cursor.0 >= self.map[self.cursor.1].len() {
            self.cursor.0 = self.map[self.cursor.1].len() - 1;
        }
    }

    /// Moves the cursor as close as possible to the give position. Cursor is moved to the last
    /// row and/or column is the X and/or Y position is too large.
    fn set_cursor(&mut self, (mut x, mut y): (usize, usize)) {
        if y >= self.map.len() {
            y = self.map.len() - 1;
        }
        self.cursor.1 = y;

        if x >= self.map[y].len() {
            x = self.map[y].len() - 1;
        }
        self.cursor.0 = x;
    }

    /// Moves the cursor with the given motion
    fn motion(&mut self, motion: Motion) {
        match motion {
            Motion::Up => self.move_up(),
            Motion::Down => self.move_down(),
            Motion::Left => self.move_left(),
            Motion::Right => self.move_right(),
        }
    }

    /// Moves the cursor one position to the left, wrapping to the start of the prior line.
    fn move_left(&mut self) {
        match &mut self.cursor {
            (0, 0) => {
                self.cursor.1 = self.map.len() - 1;
                self.cursor.0 = self.map[self.cursor.1].len() - 1;
            }
            (0, _) => {
                self.cursor.1 -= 1;
                self.cursor.0 = self.map[self.cursor.1].len() - 1;
            }
            (_, _) => {
                self.cursor.0 -= 1;
            }
        }
    }

    /// Moves the cursor one position to the right, wrapping to the start of the next line.
    fn move_right(&mut self) {
        self.cursor.0 += 1;
        if self.cursor.0 >= self.map[self.cursor.1].len() {
            self.cursor.0 = 0;
            self.cursor.1 += 1;
            if self.cursor.1 >= self.map.len() {
                self.cursor.1 = 0;
            }
        }
    }

    /// Moves the cursor one position up, wrapping to the end of the document.
    fn move_up(&mut self) {
        if self.cursor.1 == 0 {
            self.cursor.1 = self.map.len() - 1;
        } else {
            self.cursor.1 -= 1;
        }
        self.cursor.0 = std::cmp::min(self.cursor.0, self.map[self.cursor.1].len() - 1);
    }

    /// Moves the cursor one position down, wrapping to the top of the document.
    fn move_down(&mut self) {
        self.cursor.1 += 1;
        if self.cursor.1 >= self.map.len() {
            self.cursor.1 = 0;
        }
        self.cursor.0 = std::cmp::min(self.cursor.0, self.map[self.cursor.1].len() - 1);
    }
}

impl From<AllProjectsMessage> for TermAppMsg {
    fn from(value: AllProjectsMessage) -> Self {
        Self::ComponentMsg(ComponentMsg::AllProjects(value))
    }
}

impl From<BlogMessage> for TermAppMsg {
    fn from(value: BlogMessage) -> Self {
        Self::ComponentMsg(ComponentMsg::Blog(value))
    }
}

impl From<ProjectMessage> for TermAppMsg {
    fn from(value: ProjectMessage) -> Self {
        Self::ComponentMsg(ComponentMsg::Project(value))
    }
}

impl From<PostMessage> for TermAppMsg {
    fn from(value: PostMessage) -> Self {
        Self::ComponentMsg(ComponentMsg::Post(value))
    }
}
