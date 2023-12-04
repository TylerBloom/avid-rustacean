use std::sync::Mutex;

use ratatui::{prelude::*, widgets::*};

use crate::{app::CursorMap, console_log, palette::GruvboxColor};

static SCROLL_STATE: Mutex<Option<ScrollbarState>> = Mutex::new(None);

#[derive(Debug, PartialEq)]
pub struct AllProjects {
    scroll: u16,
}

#[derive(Debug, PartialEq)]
pub struct Project {}

impl Project {
    pub fn create(name: String, map: &mut CursorMap) -> Self {
        todo!()
    }

    pub fn draw(&self, frame: &mut Frame) -> Rect {
        todo!()
    }
}

impl AllProjects {
    pub fn create(map: &mut CursorMap) -> Self {
        SCROLL_STATE
            .lock()
            .unwrap()
            .insert(ScrollbarState::default());
        Self { scroll: 0 }
    }

    pub fn draw(&self, mut rect: Rect, frame: &mut Frame) -> Rect {
        console_log("Drawing projects page...");
        console_log(format!("Given area: {rect:?}"));
        let widget = Paragraph::new("A list of projects...\n".repeat(100))
            .block(Block::new().borders(Borders::all()))
            .scroll((self.scroll, 0));
        frame.render_widget(widget, rect);
        let mut state = SCROLL_STATE.lock().unwrap();
        state.insert(ScrollbarState::new(rect.height as usize));
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            rect,
            state.as_mut().unwrap(),
        );
        rect.y += rect.height;
        console_log(format!("Returned area: {rect:?}"));
        rect
    }
}
