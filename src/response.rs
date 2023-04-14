use crate::{
    id::Id,
    input::mouse::MouseButtonState,
    math_util::{Rect, VecI2},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    pub hovered: bool,
    pub buttons: [MouseButtonState; 3],
    pub id: Id,
    pub rect: Rect,
    pub mouse_pos: Option<VecI2>,
}

impl Response {
    pub fn new(rect: Rect, id: Id, mouse: Option<VecI2>) -> Self {
        Self {
            hovered: mouse.map(|m| rect.contains(m)).unwrap_or(false),
            buttons: Default::default(),
            id,
            rect,
            mouse_pos: mouse,
        }
    }
    pub fn hovered(&self) -> bool {
        self.hovered
    }

    pub fn released(&self) -> bool {
        self.buttons[0] == MouseButtonState::Up
    }

    pub fn clicked(&self) -> bool {
        self.buttons[0] == MouseButtonState::Down
    }

    pub fn pressed(&self) -> bool {
        self.buttons[0].is_down()
    }
}
