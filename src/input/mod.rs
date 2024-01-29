use crate::{containers::drop_down::DropDown, ui::Ui};

use self::{keyboard::KeyboardState, mouse::MouseState};

mod keyboard;
pub mod mouse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoreInput {
    Yes,
    No,
}

impl std::convert::From<MoreInput> for bool {
    fn from(value: MoreInput) -> Self {
        match value {
            MoreInput::Yes => true,
            MoreInput::No => false,
        }
    }
}

impl std::convert::From<bool> for MoreInput {
    fn from(value: bool) -> Self {
        match value {
            true => MoreInput::Yes,
            false => MoreInput::No,
        }
    }
}

impl std::ops::BitAnd for MoreInput {
    type Output = MoreInput;

    fn bitand(self, rhs: Self) -> Self::Output {
        (self.into() && rhs.into()).into()
    }
}

impl std::ops::BitAndAssign for MoreInput {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = ((*self).into() && rhs.into()).into()
    }
}

impl std::ops::BitOr for MoreInput {
    type Output = MoreInput;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self.into() || rhs.into()).into()
    }
}

impl std::ops::BitOrAssign for MoreInput {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = ((*self).into() || rhs.into()).into()
    }
}

#[derive(Debug, Default, Clone)]
pub struct InputState {
    pub keyboard: KeyboardState,
    pub mouse: MouseState,

    pub focused_gained: bool,
    pub focused: bool,
    pub focused_lost: bool,
}

impl InputState {
    pub fn next_state(&mut self) -> MoreInput {
        self.focused_gained = false;
        self.focused_lost = false;
        self.keyboard.next_state() & self.mouse.next_state()
    }

    pub fn ui(&self, ui: &mut Ui) {
        DropDown::new("Mouse Input")
            .default_shown(true)
            .show(ui, |ui, _| {
                self.mouse.ui(ui);
            });
        DropDown::new("Keyboard Input")
            .default_shown(true)
            .show(ui, |ui, _| {
                self.keyboard.ui(ui);
            });
    }

    pub fn handle_event(&mut self, event: crossterm::event::Event) -> MoreInput {
        use crossterm::event::*;
        match event {
            Event::Mouse(event) => self.mouse.handle_event(event),

            Event::FocusGained => {
                self.focused = true;
                self.focused_gained = true;
                MoreInput::Yes
            }
            Event::FocusLost => {
                self.focused = true;
                self.focused_lost = true;
                MoreInput::Yes
            }

            Event::Key(key) => self.keyboard.handle_key(key),
            Event::Paste(ref paste) => self.keyboard.handle_paste(paste),

            _ => MoreInput::Yes,
        }
    }
}
