use std::{cmp::Ordering, collections::HashMap};

use yew::{Component, Context, Properties};

use crate::{
    blog::Blog,
    console_debug, console_log,
    home::Home,
    posts::Post,
    project::{AllProjects, Project},
    terminal::get_window_size,
    TERMINAL,
};
use js_sys::Function;
use ratatui::{prelude::*, widgets::*};
use wasm_bindgen::prelude::Closure;
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
#[derive(Debug, Properties, PartialEq)]
pub struct TermAppProps {
    body: AppBodyProps,
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
    fn draw(&self, frame: &mut Frame) {
        match self {
            AppBody::Home(home) => home.draw(frame),
            AppBody::Projects(projects) => projects.draw(frame),
            AppBody::Project(proj) => proj.draw(frame),
            AppBody::Blog(blog) => blog.draw(frame),
            AppBody::Post(post) => post.draw(frame),
        }
    }
}

/// The different main sections the user might find themselves in.
#[derive(Debug, PartialEq)]
pub enum AppBodyProps {
    Home,
    Projects,
    Project,
    Blog,
    Post,
}

impl AppBodyProps {
    fn create_body(self, map: &mut CursorMap) -> AppBody {
        match self {
            AppBodyProps::Home => AppBody::Home(Home::create(map)),
            AppBodyProps::Projects => AppBody::Projects(AllProjects::create(map)),
            AppBodyProps::Project => AppBody::Project(Project::create(map)),
            AppBodyProps::Blog => AppBody::Blog(Blog::create(map)),
            AppBodyProps::Post => AppBody::Post(Post::create(map)),
        }
    }
}

#[derive(Debug)]
pub enum TermAppMsg {
    Resized,
}

impl TermApp {
    fn draw(&self, frame: &mut Frame) {
        self.draw_header(frame);
        self.body.draw(frame);
        self.draw_footer(frame);
    }

    fn draw_header(&self, frame: &mut Frame) {
        todo!()
    }

    fn draw_footer(&self, frame: &mut Frame) {
        // TODO: Not sure what to do here... how to get discord symbols in...
    }
}

impl Component for TermApp {
    type Message = TermAppMsg;
    type Properties = TermAppProps;

    fn create(ctx: &Context<Self>) -> Self {
        // Bind a function to the "on-resize" window event
        let cb = ctx.link().callback(|()| TermAppMsg::Resized);
        let func = move || cb.emit(());
        let func: Function = Closure::<dyn 'static + Fn()>::new(func)
            .into_js_value()
            .into();
        let window = web_sys::window().unwrap();
        window.set_onresize(Some(&func));
        // Create cursor map
        let mut cursor_map = CursorMap::new();
        cursor_map.push(String::from("home"));
        cursor_map.push(String::from("projects"));
        cursor_map.push(String::from("blog"));
        match ctx.props().body {
            AppBodyProps::Projects => {
                cursor_map.move_right();
            }
            AppBodyProps::Blog => {
                cursor_map.move_left();
            }
            AppBodyProps::Home | AppBodyProps::Project | AppBodyProps::Post => {}
        }

        // Create the viewer
        let body = ctx.props().body.create_body(&mut cursor_map);
        Self { cursor_map, body }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        console_log(format!("Got new message: {msg:?}"));
        match msg {
            TermAppMsg::Resized => {
                TERMINAL.term().backend_mut().resize_buffer();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let mut term = TERMINAL.term();
        term.draw(|frame| self.draw(frame)).unwrap();
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

    /// Gets the value that the cursor is hovering over.
    fn get_hovering(&self) -> &str {
        &self.map[self.cursor.1][self.cursor.0]
    }

    /// Moves the cursor one position to the left, wrapping to the start of the prior line.
    fn move_left(&mut self) -> &str {
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
        self.get_hovering()
    }

    /// Moves the cursor one position to the right, wrapping to the start of the next line.
    fn move_right(&mut self) -> &str {
        self.cursor.0 += 1;
        if self.cursor.0 >= self.map[self.cursor.1].len() {
            self.cursor.0 = 0;
            self.cursor.1 += 1;
            if self.cursor.1 >= self.map.len() {
                self.cursor.1 = 0;
            }
        }
        self.get_hovering()
    }

    /// Moves the cursor one position up, wrapping to the end of the document.
    fn move_up(&mut self) -> &str {
        if self.cursor.1 == 0 {
            self.cursor.1 = self.map.len() - 1;
        } else {
            self.cursor.1 -= 1;
        }
        self.cursor.0 = std::cmp::min(self.cursor.0, self.map[self.cursor.1].len() - 1);
        self.get_hovering()
    }

    /// Moves the cursor one position down, wrapping to the top of the document.
    fn move_down(&mut self) -> &str {
        self.cursor.1 += 1;
        if self.cursor.1 >= self.map.len() {
            self.cursor.1 = 0;
        }
        self.cursor.0 = std::cmp::min(self.cursor.0, self.map[self.cursor.1].len() - 1);
        self.get_hovering()
    }
}
