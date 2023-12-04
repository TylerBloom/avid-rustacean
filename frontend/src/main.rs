#![allow(unused, dead_code)]

use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    sync::{Mutex, OnceLock, MutexGuard},
};

use app::{TermApp, AppBodyProps};
use posts::Post;
use ratatui::prelude::Terminal;
use send_wrapper::SendWrapper;
use terminal::WebTerm;
use yew::{Html, html, function_component};
use yew_router::prelude::*;

pub mod posts;
pub mod terminal;
pub mod palette;
pub mod app;
pub mod home;
pub mod project;
pub mod blog;

pub static TERMINAL: Renderer = Renderer::new();

pub struct Renderer(OnceLock<Mutex<SendWrapper<Terminal<WebTerm>>>>);

impl Renderer {
    /// Construct the terminal renderer.
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }

    /// Constructs the terminal renderer around a web term.
    pub fn load(&self) {
        self.0
            .set(Mutex::new(SendWrapper::new(Terminal::new(WebTerm::new()).unwrap())))
            .unwrap();
    }

    /// Get access to the terminal renderer.
    pub fn term(&'static self) -> impl 'static + DerefMut<Target = Terminal<WebTerm>> {
        TermDeref(self.0.get().unwrap().lock().unwrap())
    }
}

struct TermDeref<'a>(MutexGuard<'a, SendWrapper<Terminal<WebTerm>>>);

impl<'a> Deref for TermDeref<'a> {
    type Target = Terminal<WebTerm>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for TermDeref<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn console_debug(s: impl Debug) {
    web_sys::console::log_1(&format!("{s:?}").into())
}

pub fn console_log(s: impl Display) {
    web_sys::console::log_1(&format!("{s}").into())
}

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    AllProjects,
    #[at("/projects/:name")]
    Project { name: String },
    #[at("/blog")]
    Blog,
    #[at("/blog/:name")]
    Post { name: String },
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <TermApp body = { AppBodyProps::Home } /> },
        Route::AllProjects => html! { <TermApp body = { AppBodyProps::AllProjects } /> },
        Route::Project { name } => html! { <TermApp body = { AppBodyProps::Project(name) } /> },
        Route::Blog => html! { <TermApp body = { AppBodyProps::Blog } /> },
        Route::Post { name } => html! { <TermApp body = { AppBodyProps::Post(name) } /> },
    }
}

#[function_component]
#[allow(non_snake_case)]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    // Load the webterm "terminal" and ratatui renderer
    TERMINAL.load();
    // Render the app
    yew::Renderer::<App>::new().render();
}
