use crate::{RelativePos, Direction, Mode};
use keymap::KeyMap;
use termion::event::Key;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
pub enum Action {
    Composite(Vec<Action>),
    DeleteChar(Direction),
    InsertChar(char),
    SetMode(Mode),
    MoveCursor(RelativePos),
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorDown,
    MoveCursorPageUp,
    MoveCursorPageDown,
    MoveCursorHome,
    MoveCursorEnd,
    Search,
    Quit,
    Exit,
    Save,
    None,
}

#[derive(Debug, Deserialize)]
struct KeyAction {
    keys: Vec<KeyMap>,
    actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
pub struct KeyMapConfig {
    normal: Vec<KeyAction>,
    insert: Vec<KeyAction>,
    visual: Vec<KeyAction>,
    command: Vec<KeyAction>,
}


impl From<&str> for KeyMapConfig {
    fn from(config: &str) -> Self {
        toml::from_str(config).unwrap()
    }
}