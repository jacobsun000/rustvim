mod editor;
mod terminal;

use editor::Editor;
pub use editor::Pos;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
