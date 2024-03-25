mod action;
mod document;
mod editor;
mod filetype;
mod highlight;
mod row;
mod mode;
mod terminal;

pub use action::Action;
pub use document::Document;
pub use editor::{Direction, Pos, RelativePos};
pub use filetype::FileType;
pub use row::Row;
pub use mode::Mode;
pub use terminal::Terminal;
use editor::Editor;

fn main() {
    Editor::default().run();
}
