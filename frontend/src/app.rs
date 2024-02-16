use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use webatui::{prelude::*, ScrollMotion};
use yew::{Component, Context, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    blog::{Blog, BlogMessage},
    home::{Home, HomeMessage},
    palette::{GruvboxColor, GruvboxExt},
    posts::{Post, PostMessage},
    project::{AllProjects, AllProjectsMessage, ProjectMessage, ProjectView},
    utils::{padded_title, ScrollRef},
    Route,
};
use derive_more::From;
use ratatui::{prelude::*, widgets::*};
use yew::prelude::*;

/// This module contains all of the machinery to run the UI app. The UI app is a single page
/// application consisting of the header, body, and footer. The body is changed when switching
/// between tabs/"pages". The app holds all of the logic for interacting with the browser window,
/// including switching between tabs and tracking the cursor.

#[derive(Debug, Clone, PartialEq)]
pub struct TermApp {
    /// The body of the UI
    body: AppBody,
}

impl TerminalApp for TermApp {
    type Message = TermAppMsg;

    fn setup(&mut self, ctx: &Context<WebTerminal<Self>>) {
        self.body.setup(ctx);
    }

    fn scroll(&mut self, scroll: ScrollMotion) -> bool {
        self.body.handle_scroll(scroll);
        true
    }

    fn update(&mut self, ctx: TermContext<'_, Self>, msg: Self::Message) -> bool {
        match msg {
            TermAppMsg::ComponentMsg(msg) => self.body.update(ctx, msg),
            TermAppMsg::Clicked(page) => {
                match &page {
                    AppBodyProps::Home => ctx.ctx().link().navigator().unwrap().push(&Route::Home),
                    AppBodyProps::AllProjects => ctx
                        .ctx()
                        .link()
                        .navigator()
                        .unwrap()
                        .push(&Route::AllProjects),
                    AppBodyProps::Blog => ctx.ctx().link().navigator().unwrap().push(&Route::Blog),
                    AppBodyProps::Project(name) => {
                        ctx.ctx().link().navigator().unwrap().push(&Route::Project {
                            name: name.to_owned(),
                        });
                    }
                    AppBodyProps::Post(name) => {
                        ctx.ctx().link().navigator().unwrap().push(&Route::Post {
                            name: name.to_owned(),
                        });
                    }
                }
                self.body = page.create_body();
                self.body.setup(ctx.ctx())
            }
        }
        true
    }

    fn render(&self, area: Rect, frame: &mut Frame<'_>) {
        self.draw(area, frame);
    }

    fn hydrate(
        &self,
        ctx: &Context<WebTerminal<Self>>,
        span: &mut webatui::backend::DehydratedSpan,
    ) {
        match span.text().trim() {
            "Home" => span.on_click(
                ctx.link()
                    .callback(|_| WebTermMessage::new(AppBodyProps::Home)),
            ),
            "Projects" => span.on_click(
                ctx.link()
                    .callback(|_| WebTermMessage::new(AppBodyProps::AllProjects)),
            ),
            "Blog" => span.on_click(
                ctx.link()
                    .callback(|_| WebTermMessage::new(AppBodyProps::Blog)),
            ),
            "Repo" => span.hyperlink("https://github.com/TylerBloom/avid-rustacean".to_owned()),
            "Email" => span.hyperlink("mailto:tylerbloom2222@gmail.com".to_owned()),
            "GitHub" => span.hyperlink("https://github.com/TylerBloom".to_owned()),
            "LinkedIn" => {
                span.hyperlink("https://www.linkedin.com/in/tyler-bloom-aba0a4156/".to_owned())
            }
            _ => self.body.hydrate(ctx, span),
        }
    }
}

/// The body used for the app on construction.
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct TermAppProps {
    pub body: AppBodyProps,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppBody {
    inner: AppBodyInner,
    // TODO: Rename to scroll
    scroll: ScrollRef,
}

/// The different main sections the user might find themselves in.
#[derive(Debug, PartialEq, From, Clone)]
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

    fn setup(&mut self, ctx: &Context<WebTerminal<TermApp>>) {
        self.inner.setup(ctx)
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

    fn hydrate(&self, ctx: &Context<WebTerminal<TermApp>>, span: &mut DehydratedSpan) {
        self.inner.hydrate(ctx, span)
    }

    fn update(&mut self, ctx: TermContext<'_, TermApp>, msg: ComponentMsg) {
        self.inner.update(ctx, msg)
    }

    fn handle_scroll(&mut self, dir: ScrollMotion) {
        self.inner.handle_scroll(dir);
        match dir {
            ScrollMotion::Up => self.scroll.next(),
            ScrollMotion::Down => self.scroll.prev(),
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

    fn setup(&mut self, ctx: &Context<WebTerminal<TermApp>>) {
        match self {
            AppBodyInner::Home(inner) => inner.setup(ctx),
            AppBodyInner::AllProjects(inner) => inner.setup(ctx),
            AppBodyInner::Project(inner) => inner.setup(ctx),
            AppBodyInner::Blog(inner) => inner.setup(ctx),
            AppBodyInner::Post(inner) => inner.setup(ctx),
        }
    }

    fn hydrate(&self, ctx: &Context<WebTerminal<TermApp>>, span: &mut DehydratedSpan) {
        match self {
            Self::Home(home) => home.hydrate(ctx, span),
            Self::AllProjects(projects) => projects.hydrate(ctx, span),
            Self::Project(proj) => proj.hydrate(ctx, span),
            Self::Blog(blog) => blog.hydrate(ctx, span),
            Self::Post(post) => post.hydrate(ctx, span),
        }
    }

    fn update(&mut self, ctx: TermContext<'_, TermApp>, msg: ComponentMsg) {
        match (self, msg) {
            (Self::Home(body), ComponentMsg::Home(msg)) => body.update(msg),
            (Self::AllProjects(body), ComponentMsg::AllProjects(msg)) => body.update(ctx, msg),
            (Self::Project(body), ComponentMsg::Project(msg)) => body.update(msg),
            (Self::Blog(body), ComponentMsg::Blog(msg)) => body.update(ctx, msg),
            (Self::Post(body), ComponentMsg::Post(msg)) => body.update(msg),
            _ => unreachable!("How did you get here? Open a PR, please"),
        }
    }

    fn handle_scroll(&mut self, dir: ScrollMotion) {
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
    fn create_body(self) -> AppBody {
        let inner = match self {
            AppBodyProps::Home => AppBodyInner::Home(Home::create()),
            AppBodyProps::AllProjects => AppBodyInner::AllProjects(AllProjects::create()),
            AppBodyProps::Project(name) => AppBodyInner::Project(ProjectView::create(name)),
            AppBodyProps::Blog => AppBodyInner::Blog(Blog::create()),
            AppBodyProps::Post(name) => AppBodyInner::Post(Post::create(name)),
        };
        AppBody::new(inner)
    }
}

#[derive(Debug, From)]
pub enum TermAppMsg {
    Clicked(AppBodyProps),
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
    pub fn new(props: AppBodyProps) -> Self {
        Self {
            body: props.create_body(),
        }
    }

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
            Span::styled("LinkedIn", GruvboxColor::blue().fg_style().to_hydrate()),
            Span::from(" "),
        ])
        .alignment(Alignment::Right);
        let tabs = Paragraph::new(line)
            .block(Block::default().borders(Borders::ALL))
            .style(GruvboxColor::orange().fg_style());
        frame.render_widget(tabs, rect);
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
