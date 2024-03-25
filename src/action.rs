use crate::{RelativePos, Direction, Mode};

pub enum Action {
    MoveCursor(RelativePos),
    DeleteChar(Direction),
    InsertChar(char),
    SetMode(Mode),
    Search,
    Quit,
    Exit,
    Save,
    None,
}