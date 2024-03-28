use crate::terminal::CursorShape;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    Command,
}

impl Mode {
    pub fn name(self) -> String {
        match self {
            Self::Normal => "Normal",
            Self::Insert => "Insert",
            Self::Visual => "Visual",
            Self::Command => "Command",
        }.to_string()
    }
    
    pub fn cursor_shape(self) -> CursorShape {
        match self {
            Self::Normal => CursorShape::SteadyBlock,
            Self::Insert => CursorShape::SteadyBar,
            Self::Visual => CursorShape::SteadyBlock,
            Self::Command => CursorShape::SteadyBar,
        }
    }
}