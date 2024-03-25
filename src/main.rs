mod document;
mod editor;
mod filetype;
mod highlight;
mod row;
mod mode;
mod terminal;

pub use document::Document;
use editor::Editor;
pub use editor::{Direction, Pos};
pub use filetype::FileType;
pub use row::Row;
pub use mode::Mode;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
