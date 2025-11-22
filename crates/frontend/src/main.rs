#![warn(rust_2018_idioms)]
#![deny(
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

use std::fmt::Debug;

use app::{AppBodyProps, TermApp};
use base16_palettes::{
    palettes::{GruvboxDarkHard, GruvboxPalette},
    Palette,
};
use webatui::{WebTermProps, WebTerminal};
use yew::{function_component, html, Html};
use yew_router::prelude::*;

pub mod app;
pub mod blog;
pub mod home;
pub mod palette;
pub mod posts;
pub mod project;
pub mod utils;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[at("/tui/")]
    Home,
    #[at("/tui/projects")]
    AllProjects,
    #[at("/tui/blog")]
    Blog,
    #[at("/tui/blog/:name")]
    Post { name: String },
}

fn switch(route: Route) -> Html {
    let body = match route {
        Route::Home => AppBodyProps::Home,
        Route::AllProjects => AppBodyProps::AllProjects,
        Route::Blog => AppBodyProps::Blog,
        Route::Post { name } => AppBodyProps::Post(name),
    };
    let inner = TermApp::new(body);
    let props = WebTermProps::new_with_palette(
        inner,
        Palette::GruvboxPalette(GruvboxPalette::GruvboxDarkHard(GruvboxDarkHard)),
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
