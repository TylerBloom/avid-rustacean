#![warn(rust_2018_idioms)]
#![allow(
    rustdoc::broken_intra_doc_links,
    unreachable_pub,
    unreachable_patterns,
    unused,
    unused_qualifications,
    while_true,
    trivial_casts,
    trivial_bounds,
    trivial_numeric_casts,
    unconditional_panic,
    clippy::all
)]

use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    sync::{Mutex, MutexGuard, OnceLock},
};

use app::{AppBodyProps, TermApp};
use ratatui::prelude::*;
use send_wrapper::SendWrapper;
use terminal::WebTerm;
use webatui::{
    palette::gruvbox::{DarkHard, GruvboxPalette},
    palette::Palette,
    WebTermProps, WebTerminal,
};
use yew::{function_component, html, Html};
use yew_router::prelude::*;

use crate::app::TermAppProps;

pub mod app;
pub mod blog;
pub mod home;
pub mod palette;
pub mod posts;
pub mod project;
pub mod terminal;
pub mod touch_scroll;
pub mod utils;

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
    let body = match route {
        Route::Home => AppBodyProps::Home,
        Route::AllProjects => AppBodyProps::AllProjects,
        Route::Project { name } => AppBodyProps::Project(name),
        Route::Blog => AppBodyProps::Blog,
        Route::Post { name } => AppBodyProps::Post(name),
    };
    let inner = TermApp::new(body);
    let props = WebTermProps::new_with_palette(
        inner,
        Palette::GruvboxPalette(GruvboxPalette::DarkHard(DarkHard)),
    );
    html! { <WebTerminal<TermApp> ..props /> }
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
    // Render the app
    yew::Renderer::<App>::new().render();
}
