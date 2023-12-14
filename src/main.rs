#![warn(clippy::all, clippy::pedantic)]

mod editor;
use editor::Editor;

fn main() {
    // Do this instead of creating a mutable variable called editor, then
    // calling editor.run();
    Editor::new().run();
}
