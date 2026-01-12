#![warn(clippy::all, clippy::pedantic)]

use std::path::PathBuf;

use clap::{arg, command, value_parser};
use cord::editor;

fn main() {
    let matches = command!()
        .arg(arg!([filename]).value_parser(value_parser!(PathBuf)))
        .get_matches();

    let filename = matches.get_one::<PathBuf>("filename");

    editor::Editor::default().run(filename);
}
