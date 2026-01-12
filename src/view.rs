use std::{io, path::PathBuf};

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

    /// # Errors
    pub fn load(&mut self, path: &PathBuf) -> Result<(), io::Error> {
        let res = std::fs::read_to_string(path);
        if let Ok(content) = res {
            for (index, content) in content.lines().enumerate() {
                self.buf.set_line_content(index, content.to_owned());
            }
            self.buf.drain(content.lines().count()..);
        }

        Ok(())
    }

    /// # Errors
    pub fn render(&self) -> Result<(), io::Error> {
        for line in self.buf.iter() {
            Terminal::clear_line()?;
            Terminal::print(line)?;
            Terminal::print("\r\n")?;
        }

        Ok(())
    }
}
