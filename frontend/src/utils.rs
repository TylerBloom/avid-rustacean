use std::{cell::RefCell, collections::HashMap};

use avid_rustacean_model::{GruvboxColor, MdNode, ParsedCode};
use ratatui::{
    prelude::*,
    widgets::{block::Title, *},
};
use yew::Context;

use crate::{
    app::TermApp,
    palette::GruvboxExt,
    terminal::{get_raw_screen_size, DehydratedSpan, NeedsHydration},
};

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
    pub fn render_scroll(&self, frame: &mut Frame<'_>, bar: Scrollbar<'_>, rect: Rect) {
        self.set_view_length(rect.height as usize);
        frame.render_stateful_widget(bar, rect, &mut self.state.borrow_mut());
    }

    /// Sets the number of lines of content to be displayed
    pub fn set_content_length(&self, lines: usize) {
        *self.content_length.borrow_mut() = lines;
        let state = self.state.borrow_mut().content_length(lines);
        *self.state.borrow_mut() = state;
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
    title: String,
    /// The lines of the paragraph
    lines: Vec<MdLine>,
    /// Any links contained within the document
    links: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MdLine {
    Plain(Line<'static>),
    Code(Line<'static>),
}

impl MdLine {
    fn as_line(&self, width: usize) -> Line<'static> {
        match self {
            MdLine::Plain(line) => line.clone(),
            MdLine::Code(code) => {
                let mut code = code.clone();
                let len = code.spans.iter().fold(0, |acc, s| acc + s.width());
                if len == 0 {
                    code.spans = vec![
                        Span::styled(" ".repeat(width), GruvboxColor::dark_3().bg_style()),
                        Span::styled("\u{200b}", GruvboxColor::dark_3().bg_style()),
                    ];
                } else {
                    code.spans.push(Span::styled(
                        " ".repeat(width.saturating_sub(len % width)),
                        GruvboxColor::dark_3().bg_style(),
                    ));
                }
                code
            }
        }
    }
}

impl Markdown {
    pub fn new(title: String, md: avid_rustacean_model::Markdown) -> Self {
        let mut links = HashMap::new();
        let widgets = render_markdown(md, &mut links);
        Self {
            lines: widgets,
            links,
            title,
        }
    }

    pub fn hydrate(&self, _ctx: &Context<TermApp>, span: &mut DehydratedSpan) {
        if let Some(link) = self.links.get(span.text()) {
            span.hyperlink(link.clone());
        }
    }

    fn get_para(&self, width: usize) -> Paragraph<'static> {
        let lines: Vec<_> = self.lines.iter().map(|l| l.as_line(width)).collect();
        Paragraph::new(lines)
            .block(
                Block::new()
                    .title(padded_title(
                        self.title.clone(),
                        GruvboxColor::yellow()
                            .full_style(GruvboxColor::dark_3())
                            .bold(),
                    ))
                    .border_style(GruvboxColor::pink().fg_style())
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all()),
            )
            .wrap(Wrap { trim: false })
    }

    pub fn draw(&self, scroll: &ScrollRef, rect: Rect, frame: &mut Frame<'_>) {
        let chunks = if is_mobile() {
            Layout::new(
                Direction::Horizontal,
                [
                    Constraint::Percentage(0),
                    Constraint::Percentage(100),
                    Constraint::Percentage(0),
                ],
            )
            .split(rect)
        } else {
            Layout::new(
                Direction::Horizontal,
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(50),
                    Constraint::Percentage(25),
                ],
            )
            .split(rect)
        };
        let para = self.get_para(chunks[1].width.saturating_sub(2) as usize);
        scroll.set_content_length(para.line_count(chunks[1].width.saturating_sub(2)));
        let view_start = scroll.view_start();
        frame.render_widget(para.scroll((view_start as u16, 0)), chunks[1]);
    }
}

/// Renders an markdown document and returns the number of lines needed to display it
pub fn render_markdown(
    md: avid_rustacean_model::Markdown,
    links: &mut HashMap<String, String>,
) -> Vec<MdLine> {
    let mut lines = vec![MdLine::Plain(Line::raw(""))];
    for node in md.0.into_iter() {
        match node {
            MdNode::Paragraph(nodes) => {
                lines.push(MdLine::Plain(render_paragraph(nodes, links)));
                lines.push(MdLine::Plain(Line::raw("\n")));
            }
            MdNode::Code(code) => {
                lines.extend(render_code(code).into_iter());
                lines.push(MdLine::Plain(Line::raw("\n")));
            }
            MdNode::BlockQuote(block) => lines.push(MdLine::Plain(Line::styled(
                block,
                GruvboxColor::orange().full_style(GruvboxColor::dark_3()),
            ))),
            MdNode::Heading(text) => {
                let line = Line::styled(
                    format!("<----- {text} ----->"),
                    GruvboxColor::yellow().fg_style(),
                )
                .alignment(Alignment::Center);
                lines.push(MdLine::Plain(line));
                lines.push(MdLine::Plain(Line::raw("")));
            }
            // TODO: The rest of these should not be free standing...
            _ => unreachable!("How did you get here? Please open an issue on Github"),
        }
    }

    lines
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

fn render_paragraph(nodes: Vec<MdNode>, links: &mut HashMap<String, String>) -> Line<'static> {
    let mut spans = Vec::with_capacity(nodes.len());
    for node in nodes.into_iter() {
        match node {
            MdNode::BlockQuote(s) => spans.push(Span::styled(
                s,
                GruvboxColor::yellow().full_style(GruvboxColor::dark_3()),
            )),
            MdNode::InlineCode(s) => spans.push(Span::styled(
                s,
                GruvboxColor::burnt_orange().full_style(GruvboxColor::dark_3()),
            )),
            MdNode::Emphasis(s) => spans.push(Span::styled(s, Style::new().italic())),
            MdNode::Link(s, link) => {
                links.insert(s.clone(), link);
                spans.push(Span::styled(
                    s,
                    GruvboxColor::blue().fg_style().to_hydrate(),
                ))
            }
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

fn render_code(code: ParsedCode) -> Vec<MdLine> {
    let mut digest = Vec::new();
    let mut spans = Vec::with_capacity(code.0.len());
    for (txt, (fg, _)) in code.0 {
        let mut iter = txt.split('\n').map(ToOwned::to_owned);
        if let Some(span) = iter.next() {
            spans.push(Span::styled(span, fg.full_style(GruvboxColor::dark_3())))
        }
        for line in iter {
            digest.push(MdLine::Code(Line::from(std::mem::take(&mut spans))));
            spans.push(Span::styled(line, fg.full_style(GruvboxColor::dark_3())))
        }
    }
    if !spans.is_empty() {
        digest.push(MdLine::Code(Line::from(std::mem::take(&mut spans))));
    }
    digest
}

pub fn is_mobile() -> bool {
    get_raw_screen_size().0 < 400
}
