use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEventState, KeyModifiers};

use crate::ui::Ui;

use super::MoreInput;

// type Key = crossterm::event::KeyCode;

#[derive(Debug, Default, Clone)]
pub struct KeyboardState {
    
    pub frame_input: String,

    pub pressed: HashMap<KeyCode, (KeyModifiers, KeyEventState)>,
}

impl KeyboardState {
    pub fn ui(&self, ui: &mut Ui) {
        ui.label(format!("frame input: {:?}", self.frame_input));
        ui.label("pressed");
        ui.add_horizontal_space(1);
        for (key, (modifier, state)) in self.pressed.iter(){
            ui.label(format!("{:?}: {:?}, {:?}", key, modifier, state))
        }
    }

    pub fn next_state(&mut self) -> MoreInput {
        self.frame_input.clear();
        self.pressed.clear();
        MoreInput::Yes
    }

    pub fn get_input(&self) -> &str{
        &self.frame_input
    }

    pub fn handle_paste(&mut self, paste: &str) -> MoreInput {
        self.frame_input.push_str(paste);
        MoreInput::Yes    
    }

    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> MoreInput {
        use crossterm::event::*;
        match key.kind{
            KeyEventKind::Repeat | KeyEventKind::Press => {
                match key.code{
                    KeyCode::Enter => {
                        self.frame_input.push('\n');
                    },
                    KeyCode::Tab => {
                        self.frame_input.push('\t');
                    },
                    KeyCode::Char(char) => {
                        self.frame_input.push(char);
                    },
                    _ => {}
                }
                self.pressed.insert(key.code, (key.modifiers, key.state));
            },
            KeyEventKind::Release => {},
        }

        MoreInput::Yes    
    }
}
