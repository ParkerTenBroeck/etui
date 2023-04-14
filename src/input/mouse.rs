use crate::math_util::VecI2;

pub mod mouse_buttons {
    pub static PRIMARY: usize = 0;
    pub static MIDDLE: usize = 1;
    pub static SECONDARY: usize = 2;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MouseButtonState {
    #[default]
    Unpressed,
    Down,
    Held,
    Up,
    Drag,
}

impl MouseButtonState {
    pub fn is_down(&self) -> bool {
        match self {
            MouseButtonState::Unpressed => false,
            MouseButtonState::Down => true,
            MouseButtonState::Held => true,
            MouseButtonState::Up => false,
            MouseButtonState::Drag => true,
        }
    }

    pub fn next_state(&mut self) {
        match self {
            MouseButtonState::Unpressed => {}
            MouseButtonState::Down => *self = MouseButtonState::Held,
            MouseButtonState::Held => {}
            MouseButtonState::Up => *self = MouseButtonState::Unpressed,
            MouseButtonState::Drag => {}
        }
    }

    pub fn is_up(&self) -> bool {
        !self.is_down()
    }
}

#[derive(Debug, Default)]
pub struct MouseState {
    pub position: VecI2,
    pub buttons: [MouseButtonState; 3],
    pub scroll: i16,
}
