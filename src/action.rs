use crate::{RelativePos, Direction, Mode};

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