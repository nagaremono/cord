#![warn(clippy::all, clippy::pedantic)]
pub mod buffer;
pub mod editor;
pub mod terminal;
pub mod view;

pub fn test_out() {
    let h = "hello".repeat(3);
    dbg!(h);
}
