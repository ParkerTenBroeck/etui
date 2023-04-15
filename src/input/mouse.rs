use crate::math_util::VecI2;

pub mod mouse_buttons {
    pub static PRIMARY: usize = 0;
    pub static MIDDLE: usize = 1;
    pub static SECONDARY: usize = 2;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButtonState {
    UnPressed(Option<VecI2>),
    Down(VecI2),
    Held(VecI2),
    Released(VecI2),

    Drag { start: VecI2, current: VecI2 },
    DragReleased { start: VecI2, released: VecI2 },
}

impl Default for MouseButtonState {
    fn default() -> Self {
        Self::UnPressed(None)
    }
}

impl MouseButtonState {
    pub fn is_down(&self) -> bool {
        match self {
            MouseButtonState::UnPressed(..) => false,
            MouseButtonState::Down(..) => true,
            MouseButtonState::Held(..) => true,
            MouseButtonState::Released(..) => false,
            MouseButtonState::DragReleased { .. } => false,
            MouseButtonState::Drag { .. } => true,
        }
    }

    pub fn is_up(&self) -> bool {
        !self.is_down()
    }

    pub fn next_state(&mut self) {
        match self {
            MouseButtonState::Down(pos) => *self = MouseButtonState::Held(*pos),
            MouseButtonState::Released(pos) => *self = MouseButtonState::UnPressed(Some(*pos)),
            MouseButtonState::DragReleased { released, .. } => {
                *self = MouseButtonState::UnPressed(Some(*released))
            }
            _ => {}
        }
    }

    pub fn button_dragged(&mut self, current: VecI2) {
        match *self {
            MouseButtonState::Held(start) => *self = MouseButtonState::Drag { start, current },
            MouseButtonState::Drag { start, .. } => {
                *self = MouseButtonState::Drag { start, current }
            }
            _ => {
                panic!(
                    "Too many mouse update, Events would have dropped: {:#?}",
                    self
                )
            }
        }
    }

    pub fn button_up(&mut self, pos: VecI2) {
        match *self {
            // werid
            MouseButtonState::UnPressed(_) => {}
            MouseButtonState::Held(_) => *self = MouseButtonState::Released(pos),
            MouseButtonState::Drag { start, .. } => {
                *self = MouseButtonState::DragReleased {
                    start,
                    released: pos,
                }
            }
            _ => {
                panic!(
                    "Too many mouse update, Events would have dropped: {:#?}",
                    self
                )
            }
        }
    }

    pub fn button_down(&mut self, pos: VecI2) {
        match *self {
            MouseButtonState::UnPressed(_) => *self = MouseButtonState::Down(pos),
            _ => {
                panic!(
                    "Too many mouse update, Events would have dropped: {:#?}",
                    self
                )
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct MouseState {
    pub position: VecI2,
    pub buttons: [MouseButtonState; 3],
    pub scroll: i16,
}

// impl MouseState{
//     pub fn ui(&self, )
// }
