use std::sync::Mutex;

use ratatui::{prelude::*, widgets::*};

use crate::{app::CursorMap, console_log};

static SCROLL_STATE: Mutex<Option<ScrollbarState>> = Mutex::new(None);

#[derive(Debug, PartialEq)]
pub struct Blog {
    scroll: u16,
}

impl Blog {
    pub fn create(map: &mut CursorMap) -> Self {
        Self { scroll: 0 }
    }

    pub fn draw(&self, rect: Rect, frame: &mut Frame) -> Rect {
        draw_all_projects_screen(rect, frame)
    }
}

fn draw_all_projects_screen(mut rect: Rect, frame: &mut Frame) -> Rect {
    console_log("Drawing blog page...");
    console_log(format!("Given area: {rect:?}"));
    let widget = Paragraph::new("A list of blog posts...\n".repeat(100))
        .alignment(Alignment::Center)
        .block(Block::new().borders(Borders::all()));
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
