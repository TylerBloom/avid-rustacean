use ratatui::{
    buffer::Cell,
    prelude::{Backend, Rect},
    style::{Color, Modifier, Style},
};
use std::{borrow::Cow, io::Result};
use wasm_bindgen::JsValue;
use web_sys::{Event, MouseEvent};
use yew::{html, Callback, Html};

use crate::{console_debug, console_log, palette::*};

/// The backend used to take ratatui widgets and render them into HTML. This is achieved through a
/// three-step rendering process.
///
/// First is the text rendering step. Here, a cell grid is populated from a ratatui rendering. This
/// grid is, in essense, the grid of characters as it will show up in the broswer. Once the text is
/// rendered, it is parsed into spans for use in the second step.
///
/// Second is the hydration step. Ratatui was not meant to run in the browser, so it does not
/// natively support associating callbacks and such with widgets. The hydration process is where
/// that occurs. Certain cell modifiers are used as flags to inform the renderer that additional
/// data might be needed. This provides an opportunity for the app to inject data such as callback
/// into the spans created after the text rendering step.
///
/// Finally, once the data has had a chance to hydrate, it is rendered into HTML, cached, and
/// served.
///
/// From the user's perspective, this process only involves rendering a frame in the Ratatui
/// terminal and then calling `WebTerm::hydrate`. The HTML that is returned from this method is
/// hydrated and ready to serve.
#[derive(Debug)]
pub struct WebTerm {
    buffer: Vec<Vec<Cell>>,
    pre_hydrated: Vec<Vec<TermSpan>>,
    rendered: Html,
}

/// The intermediate representation used for the hydration process.
#[derive(Debug)]
enum TermSpan {
    /// The data is plain data that will be rendered in a styled HTML-span tag.
    Plain((Color, Color), String),
    /// The data might need to contain additional data, such as a callback. These will be yielded
    /// to the app for hydration before being rendered into an HTML-span tag.
    Dehydrated(DehydratedSpan),
}

/// A span that might need additional data such as a callback or hyperlink
#[derive(Debug, Default)]
pub struct DehydratedSpan {
    style: (Color, Color),
    text: String,
    interaction: Interaction,
}

/// A container for the different ways that a span might be interacted with.
#[derive(Debug, Default)]
struct Interaction {
    on_click: Option<Callback<MouseEvent>>,
    hyperlink: Option<String>,
}

impl DehydratedSpan {
    fn new(fg: Color, bg: Color, text: String) -> Self {
        Self {
            style: (fg, bg),
            text,
            interaction: Interaction::default(),
        }
    }

    /// Returns a reference to the inner style.
    pub fn style(&self) -> &(Color, Color) {
        &self.style
    }

    /// Returns a reference to the inner text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Sets the `on_click` callback for the span.
    pub fn on_click(&mut self, on_click: Callback<MouseEvent>) {
        let _ = self.interaction.on_click.insert(on_click);
    }

    /// Adds a hyperlink to the span.
    pub fn hyperlink(&mut self, link: String) {
        let _ = self.interaction.hyperlink.insert(link);
    }
}

impl Default for WebTerm {
    fn default() -> Self {
        Self::new()
    }
}

impl WebTerm {
    /// The constructor for the terminal.
    pub fn new() -> Self {
        Self {
            buffer: Self::get_sized_buffer(),
            pre_hydrated: Vec::new(),
            rendered: Html::default(),
        }
    }

    fn get_sized_buffer() -> Vec<Vec<Cell>> {
        let (width, height) = get_window_size();
        vec![vec![Cell::default(); width as usize]; height as usize]
    }

    /// The method that renders the temrinal data into HTML.
    pub fn view(&mut self) -> Html {
        self.rendered.clone()
    }

    /// The rendering process is split into three steps.
    fn prerender(&mut self) {
        /// Tracks changes to the needs for hydration. If a cell is marked with the modifier
        /// RAPID_BLINK, then it will be hydrated
        enum HydrationSwitch {
            Plain,
            ToHydrate,
        }

        impl HydrationSwitch {
            /// Constructor
            fn new(modifier: &Modifier) -> Self {
                if modifier.contains(Modifier::RAPID_BLINK) {
                    Self::ToHydrate
                } else {
                    Self::Plain
                }
            }

            /// Checks to see if the next cell has different hydration needs.
            fn changes(&self, style: &Style) -> bool {
                match self {
                    HydrationSwitch::Plain
                        if style.add_modifier.contains(Modifier::RAPID_BLINK) =>
                    {
                        true
                    }
                    HydrationSwitch::ToHydrate
                        if style.sub_modifier.contains(Modifier::RAPID_BLINK) =>
                    {
                        true
                    }
                    _ => false,
                }
            }

            fn switch(&mut self) {
                match self {
                    HydrationSwitch::Plain => *self = HydrationSwitch::ToHydrate,
                    HydrationSwitch::ToHydrate => *self = HydrationSwitch::Plain,
                }
            }

            /// Creates a terminal span.
            fn create_span(&self, fg: Color, bg: Color, text: &str) -> TermSpan {
                match self {
                    HydrationSwitch::Plain => TermSpan::Plain((fg, bg), text.to_owned()),
                    HydrationSwitch::ToHydrate => {
                        TermSpan::Dehydrated(DehydratedSpan::new(fg, bg, text.to_owned()))
                    }
                }
            }
        }

        let Some(cell) = self.buffer.first().and_then(|l| l.first()) else {
            return;
        };

        let mut fg = cell.fg;
        let mut bg = cell.bg;
        let mut dehydrated = HydrationSwitch::new(&cell.style().add_modifier);
        for line in self.buffer.iter() {
            let mut text = String::with_capacity(line.len());
            let mut line_buf: Vec<TermSpan> = Vec::new();
            for c in line {
                if c.skip {
                    continue;
                }
                if c.fg != fg || c.bg != bg || dehydrated.changes(&c.style()) {
                    // Create a new node, clear the text buffer, update the foreground/background
                    if !text.is_empty() {
                        line_buf.push(dehydrated.create_span(fg, bg, &text));
                    }
                    dehydrated.switch();
                    fg = c.fg;
                    bg = c.bg;
                    text.clear();
                }
                text.push_str(c.symbol())
            }
            // Create a new node, combine into a `pre` tag, push onto buf
            if !text.is_empty() {
                line_buf.push(dehydrated.create_span(fg, bg, &text));
            }
            self.pre_hydrated.push(line_buf);
        }
    }

    pub fn hydrate<F>(&mut self, mut hydrator: F) -> Html
    where
        F: FnMut(&mut DehydratedSpan),
    {
        let mut buffer: Vec<Html> = Vec::with_capacity(self.pre_hydrated.len());
        for line in self.pre_hydrated.drain(0..) {
            let mut inner: Vec<Html> = Vec::with_capacity(line.len());
            for span in line {
                match span {
                    TermSpan::Plain((fg, bg), text) => inner.push(create_span(fg, bg, &text)),
                    TermSpan::Dehydrated(mut span) => {
                        hydrator(&mut span);
                        let DehydratedSpan {
                            style: (fg, bg),
                            text,
                            interaction,
                        } = span;
                        let Interaction {
                            on_click,
                            hyperlink,
                        } = interaction;
                        let mut element = create_span_with_callback(fg, bg, &text, on_click);
                        if let Some(link) = hyperlink {
                            element = html! { <a href = { link }> { element } </a> };
                        }
                        inner.push(element);
                    }
                }
            }
            buffer.push(html! { <pre> { for inner.drain(0..) } </pre> })
        }
        html! { <div id="the_terminal"> { for buffer.into_iter() } </div> }
    }

    pub fn resize_buffer(&mut self) {
        let (width, height) = get_window_size();
        if self.buffer.len() != height as usize || self.buffer[0].len() != width as usize {
            // Reset the buffer only if the size is actually different
            self.buffer = Self::get_sized_buffer();
        }
    }
}

impl Backend for WebTerm {
    fn draw<'a, I>(&mut self, content: I) -> Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>,
    {
        for (x, y, cell) in content {
            let y = y as usize;
            let x = x as usize;
            let line = &mut self.buffer[y];
            line.extend(std::iter::repeat_with(Cell::default).take(x.saturating_sub(line.len())));
            line[x] = cell.clone();
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<()> {
        // TODO: Actually implement
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<()> {
        todo!()
    }

    fn get_cursor(&mut self) -> Result<(u16, u16)> {
        todo!()
    }

    fn set_cursor(&mut self, _x: u16, _y: u16) -> Result<()> {
        todo!()
    }

    fn clear(&mut self) -> Result<()> {
        self.buffer = Self::get_sized_buffer();
        Ok(())
    }

    fn size(&self) -> Result<Rect> {
        let (width, height) = get_window_size();
        Ok(Rect::new(0, 0, width, height))
    }

    fn window_size(&mut self) -> Result<ratatui::backend::WindowSize> {
        todo!()
    }

    fn flush(&mut self) -> Result<()> {
        self.prerender();
        Ok(())
    }
}

fn create_span(fg: Color, bg: Color, text: &str) -> Html {
    create_span_with_callback(fg, bg, text, None)
}

fn create_span_with_callback(
    fg: Color,
    bg: Color,
    text: &str,
    cb: Option<Callback<MouseEvent>>,
) -> Html {
    let fg = to_css_color(fg).unwrap_or(GruvboxColor::default_fg().to_color_str().into());
    let bg = to_css_color(bg).unwrap_or(GruvboxColor::default_bg().to_color_str().into());
    let style = format!("color: {fg}; background-color: {bg};");
    match cb {
        Some(cb) => html! { <span style={ style } onclick = { cb }> { text } </span> },
        None => html! { <span style={ style }> { text } </span> },
    }
}

fn to_css_color(c: Color) -> Option<Cow<'static, str>> {
    match c {
        Color::Reset => None,
        Color::Black => Some("black".into()),
        Color::Red => Some("red".into()),
        Color::Green => Some("green".into()),
        Color::Yellow => Some("yellow".into()),
        Color::Blue => Some("blue".into()),
        Color::Magenta => Some("magenta".into()),
        Color::Cyan => Some("cyan".into()),
        Color::Gray => Some("gray".into()),
        Color::DarkGray => Some("darkgray".into()),
        Color::LightRed => Some("#de2b56".into()),
        Color::LightGreen => Some("lightgreen".into()),
        Color::LightYellow => Some("LightGoldenRodYellow".into()),
        Color::LightBlue => Some("LightSkyBlue".into()),
        Color::LightMagenta => Some("#ff00ff".into()),
        Color::LightCyan => Some("lightcyan".into()),
        Color::White => Some("white".into()),
        Color::Rgb(r, g, b) => Some(format!("#{r:X}{g:X}{b:X}").into()),
        Color::Indexed(c) => Some(indexed_color_str(c).into()),
    }
}

/// Calculates the number of characters that can fit in the window.
pub fn get_window_size() -> (u16, u16) {
    let (w, h) = get_raw_window_size();
    // These are mildly magical numbers... make them more precise
    (w / 10, h / 19)
}

/*
/// Calculates the number of characters that can fit in the Ratatui buffer.
pub fn get_max_window_size() -> (u16, u16) {
    let (w, h) = get_raw_window_size();
    (w / 10, u16::MAX / ( w / 10 ))
}
*/

fn get_raw_window_size() -> (u16, u16) {
    fn js_val_to_int<I: TryFrom<usize>>(val: JsValue) -> Option<I> {
        val.as_f64().and_then(|i| I::try_from(i as usize).ok())
    }

    web_sys::window()
        .and_then(|s| {
            s.inner_width()
                .ok()
                .and_then(js_val_to_int::<u16>)
                .zip(s.inner_height().ok().and_then(js_val_to_int::<u16>))
        })
        .unwrap_or((120, 120))
}
