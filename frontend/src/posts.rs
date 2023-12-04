use js_sys::Function;
use wasm_bindgen::prelude::Closure;
use yew::prelude::*;
use ratatui::{prelude::*, widgets::*};

use crate::{console_debug, TERMINAL, console_log, terminal::get_window_size, app::CursorMap};

#[derive(Debug, Properties, PartialEq)]
pub struct Post {
}

impl Post {
    pub fn create(name: String, map: &mut CursorMap) -> Self {
        todo!()
    }

    pub fn draw(&self, frame: &mut Frame) -> Rect {
        todo!()
    }
}

