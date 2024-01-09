use std::cmp::Ordering;

use yew::{platform::spawn_local, Component, Context, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    blog::{Blog, BlogMessage},
    home::{Home, HomeMessage},
    palette::{GruvboxColor, GruvboxExt},
    posts::{Post, PostMessage},
    project::{AllProjects, AllProjectsMessage, ProjectMessage, ProjectView},
    terminal::{get_raw_window_size, DehydratedSpan, NeedsHydration},
    utils::{padded_title, ScrollRef},
    Route, TERMINAL,
};
use derive_more::From;
use js_sys::Function;
use ratatui::{prelude::*, widgets::*};
use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::TouchEvent;
use yew::prelude::*;

/// This module contains all of the machinery to run the UI app. The UI app is a single page
/// application consisting of the header, body, and footer. The body is changed when switching
/// between tabs/"pages". The app holds all of the logic for interacting with the browser window,
/// including switching between tabs and tracking the cursor.

pub struct TermApp {
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
    Project(ProjectView),
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

    fn draw(&self, chunk: Rect, frame: &mut Frame<'_>) {
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

    fn update(&mut self, ctx: &Context<TermApp>, msg: ComponentMsg) {
        self.inner.update(ctx, msg)
    }

    fn handle_scroll(&mut self, dir: bool) {
        self.inner.handle_scroll(dir);
        if dir {
            self.scroll.next()
        } else {
            self.scroll.prev()
        }
    }
}

impl AppBodyInner {
    fn draw(&self, scroll: &ScrollRef, chunk: Rect, frame: &mut Frame<'_>) {
        match self {
            Self::Home(home) => home.draw(scroll, chunk, frame),
            Self::AllProjects(projects) => projects.draw(scroll, chunk, frame),
            Self::Project(proj) => proj.draw(scroll, chunk, frame),
            Self::Blog(blog) => blog.draw(scroll, chunk, frame),
            Self::Post(post) => post.draw(scroll, chunk, frame),
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

    fn update(&mut self, ctx: &Context<TermApp>, msg: ComponentMsg) {
        match (self, msg) {
            (Self::Home(body), ComponentMsg::Home(msg)) => body.update(msg),
            (Self::AllProjects(body), ComponentMsg::AllProjects(msg)) => body.update(ctx, msg),
            (Self::Project(body), ComponentMsg::Project(msg)) => body.update(ctx, msg),
            (Self::Blog(body), ComponentMsg::Blog(msg)) => body.update(ctx, msg),
            (Self::Post(body), ComponentMsg::Post(msg)) => body.update(ctx, msg),
            _ => unreachable!("How did you get here? Open a PR, please"),
        }
    }

    fn handle_scroll(&mut self, dir: bool) {
        match self {
            Self::Home(home) => home.handle_scroll(dir),
            Self::AllProjects(projects) => projects.handle_scroll(dir),
            Self::Blog(blog) => blog.handle_scroll(dir),
            Self::Project(proj) => proj.handle_scroll(dir),
            Self::Post(post) => post.handle_scroll(dir),
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
    fn create_body(self, ctx: &Context<TermApp>) -> AppBody {
        let inner = match self {
            AppBodyProps::Home => AppBodyInner::Home(Home::create(ctx)),
            AppBodyProps::AllProjects => AppBodyInner::AllProjects(AllProjects::create(ctx)),
            AppBodyProps::Project(name) => AppBodyInner::Project(ProjectView::create(name, ctx)),
            AppBodyProps::Blog => AppBodyInner::Blog(Blog::create(ctx)),
            AppBodyProps::Post(name) => AppBodyInner::Post(Post::create(name, ctx)),
        };
        AppBody::new(inner)
    }
}

#[derive(Debug, From)]
pub enum TermAppMsg {
    Resized,
    Clicked(AppBodyProps),
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
    Home(HomeMessage),
    AllProjects(AllProjectsMessage),
    Blog(BlogMessage),
    Project(ProjectMessage),
    Post(PostMessage),
}

impl TermApp {
    fn draw(&self, area: Rect, frame: &mut Frame<'_>) {
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

    fn draw_header(&self, rect: Rect, frame: &mut Frame<'_>) {
        let titles = vec![
            Line::styled("Home", GruvboxColor::teal().fg_style().to_hydrate()),
            Line::styled("Projects", GruvboxColor::teal().fg_style().to_hydrate()),
            Line::styled("Blog", GruvboxColor::teal().fg_style().to_hydrate()),
        ];
        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(padded_title(
                        "The Avid Rustacean".to_owned(),
                        GruvboxColor::burnt_orange().fg_style(),
                    ))
                    .title_alignment(Alignment::Center),
            )
            .style(GruvboxColor::orange().full_style(GruvboxColor::dark_2()));
        frame.render_widget(tabs, rect);
    }

    fn draw_footer(&self, rect: Rect, frame: &mut Frame<'_>) {
        let line = Line::from(vec![
            Span::styled("Email", GruvboxColor::blue().fg_style().to_hydrate()),
            Span::from(" | "),
            Span::styled("Repo", GruvboxColor::blue().fg_style().to_hydrate()),
            Span::from(" | "),
            Span::styled("GitHub", GruvboxColor::blue().fg_style().to_hydrate()),
            Span::from(" | "),
            Span::styled("LinkdIn", GruvboxColor::blue().fg_style().to_hydrate()),
            Span::from(" "),
        ])
        .alignment(Alignment::Right);
        let tabs = Paragraph::new(line)
            .block(Block::default().borders(Borders::ALL))
            .style(GruvboxColor::orange().fg_style());
        frame.render_widget(tabs, rect);
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
        // Bind a function to the "touch-move" window event
        // TODO: This will require quite the workaround...
        //  - Make an accumulator struct that holds the initial touch position, the latest position, and total (vertical) delta
        //  - Put this behind an Arc<Mutex>
        //  - Make touch start and move closures that get a handle
        //  - Have touch start reset the acc
        //  - Have touch move emit messages after some threshold
        // This approach is naive, but we're going for functional first
        let _cb = ctx.link().callback(|msg: TermAppMsg| msg);
        let func = move |event: JsValue| {
            let event: TouchEvent = event.into();
            spawn_local(async move {
                let mut counter = 0;
                let list = event.touches();
                let mut digest = format!("Got touch event with {} touch(es)... ", list.length());
                while let Some(touch) = list.get(counter) {
                    counter += 1;
                    digest.push_str(&format!("{touch:?} -> {:?} ", touch.target()));
                }
                gloo_net::http::Request::post("/api/v1/print")
                    .body(digest)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
                // Naive approach: Always up or down
                let Some(_) = event.target_touches().get(0) else {
                    return;
                };
            })
        };
        let func: Function = Closure::<dyn 'static + Fn(JsValue)>::new(func)
            .into_js_value()
            .into();
        window.set_ontouchmove(Some(&func));
        // Create the viewer
        let body = ctx.props().body.clone().create_body(ctx);
        Self { body }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TermAppMsg::Resized => {
                spawn_local(async move {
                    gloo_net::http::Request::post("/api/v1/print")
                        .body(format!("Got resize message: {:?}", get_raw_window_size()))
                        .unwrap()
                        .send()
                        .await
                        .unwrap();
                });
                TERMINAL.term().backend_mut().resize_buffer()
            }
            TermAppMsg::ComponentMsg(msg) => self.body.update(ctx, msg),
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
                self.body = page.create_body(ctx);
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
            "Repo" => span.hyperlink("https://github.com/TylerBloom/avid-rustacean".to_owned()),
            "Email" => span.hyperlink("mailto:tylerbloom2222@gmail.com".to_owned()),
            "GitHub" => span.hyperlink("https://github.com/TylerBloom".to_owned()),
            "LinkedIn" => {
                span.hyperlink("https://www.linkedin.com/in/tyler-bloom-aba0a4156/".to_owned())
            }
            _ => self.body.hydrate(ctx, span),
        })
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
