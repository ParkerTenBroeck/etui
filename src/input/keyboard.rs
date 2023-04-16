use crate::ui::Ui;

type Key = crossterm::event::KeyCode;

#[derive(Debug, Default, Clone, Copy)]
pub struct KeyboardState {}

impl KeyboardState {
    pub fn ui(&self, ui: &mut Ui) {}

    pub fn next_state(&mut self) {}
}
