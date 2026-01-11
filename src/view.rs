use std::io;

use crate::{
    buffer::Buffer,
    terminal::{Terminal, TerminalSize},
};

const VERSION: &str = "0.1.0";
const GREET: &str = "~ Hello, World!";

#[derive(Debug)]
pub struct View {
    buf: Buffer,
}

impl Default for View {
    fn default() -> Self {
        let TerminalSize { row, .. } =
            Terminal::size().unwrap_or(TerminalSize { row: 64, col: 64 });
        let mut buf = Buffer::default();
        buf.push(String::from(GREET));

        for _ in 1..row - 1 {
            buf.push(String::from("~"));
        }

        Self { buf }
    }
}

pub trait Renderer {
    /// # Errors
    fn render(&self) -> Result<(), io::Error>;
}

impl View {
    fn greet(&self) -> Result<(), io::Error> {
        let TerminalSize { col, .. } = Terminal::size()?;
        let message = format!("hello from cord version: {VERSION}");

        let mid = (col / 2) as usize;
        let start = mid - message.len() / 2;
        let left_pad = " ".repeat(start);
        Terminal::print(&format!("{left_pad}{message}"))?;

        Ok(())
    }
}

impl Renderer for View {
    fn render(&self) -> Result<(), io::Error> {
        for line in self.buf.iter() {
            Terminal::clear_line()?;
            Terminal::print(line)?;
            Terminal::print("\r\n")?;
        }

        Ok(())
    }
}
