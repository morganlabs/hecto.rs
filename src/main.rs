#![warn(clippy::all, clippy::pedantic)]

mod editor;
mod terminal;

pub use terminal::Terminal;

fn main() {
    let mut editor = editor::Editor::default();
    editor.run();
}
