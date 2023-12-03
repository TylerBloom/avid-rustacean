use ratatui::Frame;

use crate::app::CursorMap;


#[derive(Debug, PartialEq)]
pub struct AllProjects {
}

#[derive(Debug, PartialEq)]
pub struct Project {
}

impl Project {
    pub fn create(map: &mut CursorMap) -> Self {
        todo!()
    }

    pub fn draw(&self, frame: &mut Frame) {
        todo!()
    }
}

impl AllProjects {
    pub fn create(map: &mut CursorMap) -> Self {
        todo!()
    }

    pub fn draw(&self, frame: &mut Frame) {
        todo!()
    }
}
