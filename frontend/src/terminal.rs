use ratatui::{
    buffer::Cell,
    prelude::{Backend, Rect},
    style::Color,
};
use std::{borrow::Cow, io::Result};
use wasm_bindgen::JsValue;
use yew::{html, Html};

use crate::{console_debug, console_log, palette::*};

#[derive(Debug)]
pub struct WebTerm {
    buffer: Vec<Vec<Cell>>,
    rendered: Html,
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

    pub fn render(&mut self) -> Html {
        let mut buf: Vec<Html> = Vec::new();
        let Some(cell) = self.buffer.first().and_then(|l| l.first()) else {
            return Html::default();
        };
        let mut fg = cell.fg;
        let mut bg = cell.bg;
        for line in self.buffer.iter() {
            let mut text = String::with_capacity(line.len());
            let mut line_buf: Vec<Html> = Vec::new();
            for c in line {
                if c.skip {
                    continue;
                }
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
        }
        let digest: Html = buf.into_iter().collect();
        self.rendered = digest.clone();
        digest
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
        self.render();
        Ok(())
    }
}

fn create_span(fg: Color, bg: Color, text: &str) -> Html {
    let fg = to_css_color(fg).unwrap_or(GruvboxColor::default_fg().to_color_str().into());
    let bg = to_css_color(bg).unwrap_or(GruvboxColor::default_bg().to_color_str().into());
    let style = format!("color: {fg}; background-color: {bg};");
    html! { <span style={ style }> { text } </span> }
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
