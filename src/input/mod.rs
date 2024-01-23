use crate::ui::Ui;

use self::{keyboard::KeyboardState, mouse::MouseState};

mod keyboard;
pub mod mouse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoreInput{
    Yes,
    No
}

impl std::convert::From<MoreInput> for bool{
    fn from(value: MoreInput) -> Self {
        match value{
            MoreInput::Yes => true,
            MoreInput::No => false,
        }
    }
}

impl std::convert::From<bool> for MoreInput{
    fn from(value: bool) -> Self {
        match value{
            true => MoreInput::Yes,
            false => MoreInput::No,
        }
    }
}

impl std::ops::BitAnd for MoreInput{
    type Output = MoreInput;

    fn bitand(self, rhs: Self) -> Self::Output {
        (self.into() && rhs.into()).into()
    }
}

impl std::ops::BitAndAssign for MoreInput{
    fn bitand_assign(&mut self, rhs: Self) {
        *self = ((*self).into() && rhs.into()).into()
    }
}

impl std::ops::BitOr for MoreInput{
    type Output = MoreInput;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self.into() || rhs.into()).into()
    }
}

impl std::ops::BitOrAssign for MoreInput{
    fn bitor_assign(&mut self, rhs: Self) {
        *self = ((*self).into() || rhs.into()).into()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct InputState {
    pub keyboard: KeyboardState,
    pub mouse: MouseState,
}

impl InputState {
    pub fn next_state(&mut self) -> MoreInput{
        self.mouse.next_state()
    }

    pub fn ui(&self, ui: &mut Ui) {
        ui.drop_down("Keyboard Input", |ui| {
            self.keyboard.ui(ui);
        });

        ui.drop_down("Mouse Input", |ui| {
            self.mouse.ui(ui);
        });
    }
}
