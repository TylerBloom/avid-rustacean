use ratatui::{
    buffer::Cell,
    prelude::{Backend, Rect},
    style::Color,
};
use std::io::Result;
use wasm_bindgen::JsValue;
use yew::{html, Html};

use crate::{console_debug, console_log};

#[derive(Debug)]
pub struct WebTerm {
    buffer: Vec<Vec<Cell>>,
    ready: Html,
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
            ready: Html::default(),
        }
    }

    fn get_sized_buffer() -> Vec<Vec<Cell>> {
        let (width, height) = get_window_size();
        vec![vec![Cell::default(); width as usize]; height as usize]
    }

    /// The method that renders the temrinal data into HTML.
    pub fn view(&self) -> Html {
        // html! { <pre width={self.inner.len().to_string()}> { for self.inner.iter().map(|s| s.as_str()).chain(std::iter::once("\n")) }</pre> }
        // Turns the text green!!
        // self.inner.iter().map(|s| html! { <pre> <span class = "green">{ s }</span> </pre> }).collect()
        self.ready.clone()
    }

    pub fn resize_buffer(&mut self) {
        self.buffer = Self::get_sized_buffer();
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
        let old = std::mem::replace(&mut self.buffer, Self::get_sized_buffer());
        let mut buf: Vec<Html> = Vec::new();
        let Some(cell) = self.buffer.first().and_then(|l| l.first()) else {
            return Ok(());
        };
        let mut fg = cell.fg;
        let mut bg = cell.bg;
        for line in old {
            console_log("Starting line...");
            let mut text = String::with_capacity(line.len());
            let mut line_buf: Vec<Html> = Vec::new();
            for c in line {
                if c.fg != fg || c.bg != bg {
                    // Create a new node, clear the text buffer, update the foreground/background
                    if !text.is_empty() {
                        line_buf.push(create_span(fg, bg, &text));
                    }
                    fg = c.fg;
                    bg = c.bg;
                    text.clear();
                }
                text.push_str(&c.symbol)
            }
            // Create a new node, combine into a `pre` tag, push onto buf
            if !text.is_empty() {
                line_buf.push(create_span(fg, bg, &text));
            }
            buf.push(html! { <pre> { for line_buf } </pre> });
            console_log("End of line...");
        }
        self.ready = buf.into_iter().collect();
        Ok(())
    }
}

fn create_span(fg: Color, bg: Color, text: &str) -> Html {
    console_log(format!("Creating span: {fg} {bg} {text:?}"));
    let fg = to_css_color(fg).unwrap_or("#fbf1c7");
    let bg = to_css_color(bg).unwrap_or("#504945");
    let style = format!("color: {fg}; background-color: {bg};");
    html! { <span style={ style }> { text } </span> }
}

fn to_css_color(c: Color) -> Option<&'static str> {
    match c {
        Color::Reset => None,
        Color::Black => Some("black"),
        Color::Red => Some("red"),
        Color::Green => Some("green"),
        Color::Yellow => Some("yellow"),
        Color::Blue => Some("blue"),
        Color::Magenta => Some("magenta"),
        Color::Cyan => Some("cyan"),
        Color::Gray => Some("gray"),
        Color::DarkGray => Some("darkgray"),
        Color::LightRed => Some("#de2b56"),
        Color::LightGreen => Some("lightgreen"),
        Color::LightYellow => Some("LightGoldenRodYellow"),
        Color::LightBlue => Some("LightSkyBlue"),
        Color::LightMagenta => Some("#ff00ff"),
        Color::LightCyan => Some("lightcyan"),
        Color::White => Some("white"),
        Color::Rgb(_, _, _) => todo!(),
        Color::Indexed(_) => todo!(),
    }
}

/// Calculates the number of characters that can fit in the window.
pub fn get_window_size() -> (u16, u16) {
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
        // These are mildly magical numbers... make them more precise
        .map(|(w, h)| (w / 10, h / 19))
        .unwrap_or((120, 120))
}
