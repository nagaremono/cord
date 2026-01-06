#![warn(clippy::all, clippy::pedantic)]
pub mod editor;
pub mod terminal;

pub fn test_out() {
    let h = "hello".repeat(3);
    dbg!(h);
}
