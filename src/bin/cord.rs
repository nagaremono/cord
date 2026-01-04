#![warn(clippy::all, clippy::pedantic)]

use cord::editor;

fn main() {
    editor::Editor::default().run();
}
