use crate::ui::Ui;

use self::{keyboard::KeyboardState, mouse::MouseState};

mod keyboard;
pub mod mouse;

#[derive(Debug, Default, Clone, Copy)]
pub struct InputState {
    pub keyboard: KeyboardState,
    pub mouse: MouseState,
}

impl InputState {
    pub fn next_state(&mut self) {
        self.mouse.next_state();
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
