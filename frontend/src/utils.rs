use std::{cell::RefCell, collections::HashMap};

use avid_rustacean_model::{GruvboxColor, MdNode, ParsedCode};
use ratatui::{
    prelude::*,
    widgets::{block::Title, *},
};
use serde::Serialize;

use crate::{console_debug, console_log, palette::GruvboxExt, terminal::NeedsHydration, TERMINAL};

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
    pub fn new(content_length: usize, lines: usize) -> Self {
        Self {
            content_length: RefCell::new(content_length),
            view_length: RefCell::new(0),
            view_start: RefCell::new(0),
            state: RefCell::new(ScrollbarState::new(lines)),
        }
    }

    /// Renders the scrollbar into a frame
    pub fn render_scroll(&self, frame: &mut Frame, bar: Scrollbar, rect: Rect) {
        self.set_view_length(rect.height as usize);
        frame.render_stateful_widget(bar, rect, &mut self.state.borrow_mut());
    }

    /// Sets the number of lines of content to be displayed
    pub fn set_content_length(&self, lines: usize) {
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
    pub fn view_start(&self) -> usize {
        *self.view_start.borrow()
    }

    /// Gets the length of the view port.
    pub fn view_length(&self) -> usize {
        *self.view_length.borrow()
    }

    pub fn set_view_start(&self, view_start: usize) {
        *self.view_start.borrow_mut() = view_start;
        let state = self.state.borrow().position(view_start);
        *self.state.borrow_mut() = state;
    }

    /// Moves the scroll state down.
    pub fn next(&mut self) {
        *self.view_start.get_mut() = self
            .view_start
            .get_mut()
            .checked_add(1)
            .unwrap_or(*self.content_length.borrow());
        self.state.get_mut().next()
    }

    /// Moves the scroll state up.
    pub fn prev(&mut self) {
        *self.view_start.get_mut() = self.view_start.get_mut().saturating_sub(1);
        self.state.get_mut().prev()
    }
}

// Remember the order:
// Span -> Line -> Text (-> Paragraph)

/// A container for pre-parsing and storing markdown
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Markdown {
    /// The paragraphs that are parsed out of the markdown
    widget: Paragraph<'static>,
    /// Any links contained within the document
    links: HashMap<String, String>,
}

impl Markdown {
    pub fn new(title: String, md: avid_rustacean_model::Markdown) -> Self {
        let width = TERMINAL
            .term()
            .backend()
            .size()
            .unwrap()
            .width
            .saturating_sub(1)
            / 2;
        let mut links = HashMap::new();
        let widgets = render_markdown(title, md, width, &mut links);
        Self { widget: widgets, links }
    }

    pub fn length(&self, width: u16) -> usize {
        self.widget.line_count(width)
    }

    pub fn draw(&self, scroll: &ScrollRef, mut rect: Rect, frame: &mut Frame) {
        let chunks = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ],
        )
        .split(rect);
        scroll.set_content_length(self.widget.line_count(chunks[1].width.saturating_sub(2)));
        let mut view_start = scroll.view_start();
        frame.render_widget(self.widget.clone().scroll((view_start as u16, 0)), chunks[1]);
    }
}
/// Renders an markdown document and returns the number of lines needed to display it
pub fn render_markdown(
    title: String,
    md: avid_rustacean_model::Markdown,
    width: u16,
    links: &mut HashMap<String, String>,
) -> Paragraph<'static> {
    let mut lines = vec![Line::raw("")];
    for node in md.0.into_iter() {
        match node {
            MdNode::Paragraph(nodes) => {
                lines.push(render_paragraph(nodes));
                lines.push(Line::raw("\n"));
            },
            MdNode::Code(code) => {
                lines.extend(render_code(code, width as usize).into_iter());
                lines.push(Line::raw("\n"));
            },
            MdNode::BlockQuote(block) => lines.push(Line::styled(
                block,
                GruvboxColor::light_2().full_style(GruvboxColor::dark_1()),
            )),
            MdNode::InlineCode(code) => lines.push(Line::styled(
                code,
                GruvboxColor::light_2().full_style(GruvboxColor::dark_1()),
            )),
            MdNode::Emphasis(text) => lines.push(Line::styled(text, Style::new().italic())),
            MdNode::Strong(text) => lines.push(Line::styled(text, Style::new().bold())),
            MdNode::Heading(text) => {
                let line = Line::raw(format!("<----- {text} ----->")).alignment(Alignment::Center);
                lines.push(line);
                let line = Line::raw("");
                lines.push(line);
            },
            MdNode::Text(text) => lines.push(Line::raw(text)),
            MdNode::Break => {}
            // TODO: No idea...
            MdNode::List(nodes) => todo!(),
            // TODO: Mark for hydration
            MdNode::Link(_, _) => todo!(),
            // TODO: Not sure how to support this yet
            MdNode::ThematicBreak => {}
        }
    }

    let digest = Paragraph::new(lines)
        .block(
            Block::new()
                .title(padded_title(
                    title,
                    Style::new()
                        .bold()
                        .fg(GruvboxColor::light_4().to_color())
                        .bg(GruvboxColor::dark_3().to_color()),
                ))
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        )
        .wrap(Wrap { trim: false });

    digest
}

pub fn padded_title(title: String, style: Style) -> Title<'static> {
    vec![
        Span::from(" "),
        Span::styled(" ", style),
        Span::styled(title, style),
        Span::styled(" ", style),
        Span::from(" "),
    ]
    .into()
}

fn render_paragraph(nodes: Vec<MdNode>) -> Line<'static> {
    let mut spans = Vec::with_capacity(nodes.len());
    for node in nodes.into_iter() {
        match node {
            MdNode::BlockQuote(s) => spans.push(Span::styled(s, GruvboxColor::dark_1().bg_style())),
            MdNode::InlineCode(s) => spans.push(Span::styled(s, GruvboxColor::dark_1().bg_style())),
            MdNode::Emphasis(s) => spans.push(Span::styled(s, Style::new().italic())),
            MdNode::Link(s, _) => spans.push(Span::styled(
                s,
                GruvboxColor::blue().fg_style().to_hydrate(),
            )),
            MdNode::Strong(s) => spans.push(Span::styled(s, Style::new().bold())),
            MdNode::Text(s) => spans.push(Span::raw(s)),
            // TODO: Dunno yet
            MdNode::List(_) => todo!(),
            MdNode::ThematicBreak => todo!(),
            MdNode::Break => todo!(),
            // These won't happen
            MdNode::Heading(_) | MdNode::Paragraph(_) | MdNode::Code(_) => {}
        }
    }
    Line::from(spans)
}

fn render_code(code: ParsedCode, width: usize) -> Vec<Line<'static>> {
    let mut digest = Vec::new();
    let mut spans = Vec::with_capacity(code.0.len());
    for (txt, (fg, _)) in code.0 {
        let mut iter = txt.split('\n').map(ToOwned::to_owned);
        if let Some(span) = iter.next() {
            spans.push(Span::styled(span, fg.full_style(GruvboxColor::dark_3())))
        }
        for line in iter {
            let len = spans.iter().fold(0, |acc, s| acc + s.width());
            if len == 0 {
                spans.push(Span::styled(
                    " ".repeat(width.saturating_sub(len % width)),
                    GruvboxColor::dark_3().bg_style(),
                ));
            } else {
                spans.push(Span::styled(
                    " ".repeat(width.saturating_sub(1 + (len % width))),
                    GruvboxColor::dark_3().bg_style(),
                ));
            }
            digest.push(Line::from(std::mem::take(&mut spans)));
            spans.push(Span::styled(line, fg.full_style(GruvboxColor::dark_3())))
        }
    }
    if !spans.is_empty() {
        let len = spans.iter().fold(0, |acc, s| acc + s.width());
        if len == 0 {
            spans.push(Span::styled(
                " ".repeat(width.saturating_sub(len % width)),
                GruvboxColor::dark_3().bg_style(),
            ));
        } else {
            spans.push(Span::styled(
                " ".repeat(width.saturating_sub(1 + (len % width))),
                GruvboxColor::dark_3().bg_style(),
            ));
        }
        digest.push(Line::from(std::mem::take(&mut spans)));
    }
    digest
}
