use crate::{math_util::VecI2, ui::Ui};

use super::MoreInput;

pub mod mouse_buttons {
    pub static PRIMARY: usize = 0;
    pub static MIDDLE: usize = 1;
    pub static SECONDARY: usize = 2;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButtonState {
    UnPressed,
    Down(VecI2),
    Held(VecI2),
    Released(VecI2),

    Drag { start: VecI2, current: VecI2 },
    DragReleased { start: VecI2, released: VecI2 },
}

impl Default for MouseButtonState {
    fn default() -> Self {
        Self::UnPressed
    }
}

impl MouseButtonState {
    pub fn is_down(&self) -> bool {
        match self {
            MouseButtonState::UnPressed => false,
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

    pub fn next_state(&mut self) -> MoreInput {
        match self {
            MouseButtonState::Down(pos) => *self = MouseButtonState::Held(*pos),
            MouseButtonState::Released(..) => *self = MouseButtonState::UnPressed,
            MouseButtonState::DragReleased { .. } => *self = MouseButtonState::UnPressed,
            _ => return MoreInput::Yes,
        }
        MoreInput::No
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
            MouseButtonState::UnPressed => {}
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
            MouseButtonState::UnPressed => *self = MouseButtonState::Down(pos),
            _ => {
                panic!(
                    "Too many mouse update, Events would have dropped: {:#?}",
                    self
                )
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MouseState {
    pub position: Option<VecI2>,
    pub buttons: [MouseButtonState; 3],
    pub delta_scroll: i16,
}

impl MouseState {
    pub fn ui(&self, ui: &mut Ui) {
        ui.vertical(|ui| {
            if let Some(pos) = self.position {
                ui.label(format!("x: {}, y: {}", pos.x, pos.y))
            } else {
                ui.label("None")
            }

            fn button_ui(ui: &mut Ui, button: &MouseButtonState) {
                ui.label(format!("{button:?}"))
            }

            for (i, button) in ["Left:  ", "Middle:", "Right: "].iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(*button);
                    ui.add_space_primary_direction(1);
                    button_ui(ui, &self.buttons[i])
                });
            }

            ui.label(format!("delta scroll: {}", self.delta_scroll))
        });
    }

    pub fn next_state(&mut self) -> MoreInput {
        let mut more_input = MoreInput::Yes;
        for button in &mut self.buttons {
            more_input &= button.next_state();
        }
        self.delta_scroll = 0;
        more_input
    }

    pub fn handle_event(&mut self, event: crossterm::event::MouseEvent) -> MoreInput {
        use crossterm::event::*;
        let event_pos = VecI2::new(event.column, event.row);
        self.position = Some(event_pos);
        match event.kind {
            MouseEventKind::Down(button)
            | MouseEventKind::Up(button)
            | MouseEventKind::Drag(button) => {
                let button = match button {
                    MouseButton::Left => &mut self.buttons[0],
                    MouseButton::Right => &mut self.buttons[2],
                    MouseButton::Middle => &mut self.buttons[1],
                };
                match event.kind {
                    MouseEventKind::Down(_) => {
                        button.button_down(event_pos);
                    }
                    MouseEventKind::Up(_) => {
                        button.button_up(event_pos);
                    }
                    MouseEventKind::Drag(_) => {
                        button.button_dragged(event_pos);
                    }
                    _ => {}
                }

                MoreInput::No
            }
            MouseEventKind::Moved => MoreInput::Yes,
            MouseEventKind::ScrollDown => {
                self.delta_scroll -= 1;
                MoreInput::Yes
            }
            MouseEventKind::ScrollUp => {
                self.delta_scroll += 1;
                MoreInput::Yes
            }
        }
    }
}
