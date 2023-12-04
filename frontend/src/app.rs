use std::{cmp::Ordering, collections::HashMap};

use yew::{Component, Context, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    blog::Blog,
    console_debug, console_log,
    home::Home,
    palette::GruvboxColor,
    posts::Post,
    project::{AllProjects, Project},
    terminal::get_window_size,
    Route, TERMINAL,
};
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

/// The different main sections the user might find themselves in.
#[derive(Debug, PartialEq)]
pub enum AppBody {
    Home(Home),
    Projects(AllProjects),
    Project(Project),
    Blog(Blog),
    Post(Post),
}

impl AppBody {
    fn draw(&self, chunk: Rect, frame: &mut Frame) -> Rect {
        console_log(format!("Drawing body: {chunk:?}"));
        match self {
            AppBody::Home(home) => home.draw(chunk, frame),
            AppBody::Projects(projects) => projects.draw(chunk, frame),
            AppBody::Project(proj) => proj.draw(frame),
            AppBody::Blog(blog) => blog.draw(chunk, frame),
            AppBody::Post(post) => post.draw(frame),
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
    fn create_body(self, map: &mut CursorMap) -> AppBody {
        match self {
            AppBodyProps::Home => AppBody::Home(Home::create(map)),
            AppBodyProps::AllProjects => AppBody::Projects(AllProjects::create(map)),
            AppBodyProps::Project(name) => AppBody::Project(Project::create(name, map)),
            AppBodyProps::Blog => AppBody::Blog(Blog::create(map)),
            AppBodyProps::Post(name) => AppBody::Post(Post::create(name, map)),
        }
    }
}

#[derive(Debug)]
pub enum TermAppMsg {
    Resized,
    Entered,
    MovedUp,
    MovedLeft,
    MovedRight,
    MovedDown,
}

impl TermApp {
    fn draw(&self, area: Rect, frame: &mut Frame) {
        let (width, height) = get_window_size();
        let chunk = Rect {
            x: 0,
            y: 0,
            width: area.width,
            height: 3,
        };
        self.draw_header(chunk, frame);
        let chunk = Rect {
            x: 0,
            y: 3,
            width: area.width,
            height: area.height.saturating_sub(6),
        };
        let body = self.body.draw(chunk, frame);
        console_log("Body drawn!!");
        let chunk = Rect {
            x: 0,
            y: body.y,
            width: area.width,
            height: 3,
        };
        self.draw_footer(chunk, frame);
    }

    fn draw_header(&self, rect: Rect, frame: &mut Frame) {
        let titles = vec!["Home", "Projects", "Blog"];
        let mut tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL))
            .style(GruvboxColor::teal().full_style(GruvboxColor::dark_2()))
            .highlight_style(GruvboxColor::green().full_style(GruvboxColor::dark_3()));
        if let (x, 0) = self.cursor_map.get_position() {
            tabs = tabs.select(x);
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
        // Bind a function to the "on-keydown" window event
        let cb = ctx.link().callback(|msg: TermAppMsg| msg);
        let func = move |event: JsValue| {
            let event: KeyboardEvent = event.into();
            match event.key().as_str() {
                "ArrowUp" => cb.emit(TermAppMsg::MovedUp),
                "ArrowDown" => cb.emit(TermAppMsg::MovedDown),
                "ArrowLeft" => cb.emit(TermAppMsg::MovedLeft),
                "ArrowRight" => cb.emit(TermAppMsg::MovedRight),
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
        let body = ctx.props().body.clone().create_body(&mut cursor_map);
        Self { cursor_map, body }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        console_log(format!("Got message: {msg:?}"));
        match msg {
            TermAppMsg::Resized => TERMINAL.term().backend_mut().resize_buffer(),
            TermAppMsg::Entered => match self.cursor_map.get_position() {
                (0, 0) => {
                    ctx.link().navigator().unwrap().push(&Route::Home);
                    self.body = AppBody::Home(Home::create(&mut self.cursor_map));
                }
                (1, 0) => {
                    ctx.link().navigator().unwrap().push(&Route::AllProjects);
                    self.body = AppBody::Projects(AllProjects::create(&mut self.cursor_map));
                }
                (2, 0) => {
                    ctx.link().navigator().unwrap().push(&Route::Blog);
                    self.body = AppBody::Blog(Blog::create(&mut self.cursor_map));
                }
                _ => todo!(),
            },
            TermAppMsg::MovedUp => self.cursor_map.move_up(),
            TermAppMsg::MovedLeft => self.cursor_map.move_left(),
            TermAppMsg::MovedRight => self.cursor_map.move_right(),
            TermAppMsg::MovedDown => self.cursor_map.move_down(),
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let mut term = TERMINAL.term();
        let area = term.size().unwrap();
        console_log(format!("Size of the terminal: {area:?}"));
        term.draw(|frame| self.draw(area, frame)).unwrap();
        term.backend_mut().view()
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
    fn push(&mut self, next: String) {
        self.map.last_mut().unwrap().push(next)
    }

    /// Adds a new, blank line onto which this item and all subsequent item will be pushed.
    fn append_and_push(&mut self, next: String) {
        self.map.push(Vec::new());
        self.push(next)
    }

    /// Returns the current position of the cursor
    fn get_position(&self) -> (usize, usize) {
        self.cursor
    }

    /// Gets the value that the cursor is hovering over.
    fn get_hovering(&self) -> &str {
        &self.map[self.cursor.1][self.cursor.0]
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
