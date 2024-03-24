mod document;
mod editor;
mod filetype;
mod highlighting;
mod row;
mod terminal;

pub use document::Document;
use editor::Editor;
pub use editor::{Pos, SearchDirection};
pub use filetype::{FileType, HighlightingOptions};
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
