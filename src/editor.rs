use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, read},
    terminal::ClearType,
};
use std::io;

use crate::terminal::{CursorPos, Terminal, TerminalSize};

const VERSION: &str = "0.1.0";

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    /// # Panics
    pub fn run(&mut self) {
        Terminal::init().unwrap();
        let result = self.repl();
        Terminal::close().unwrap();
        result.unwrap();
    }

    /// # Errors
    pub fn repl(&mut self) -> Result<(), io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }

            let event = read()?;
            self.eval_event(&event);
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        Terminal::begin()?;
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear(ClearType::CurrentLine)?;
            Terminal::print("Goodbye...\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor(&CursorPos { row: 0, col: 0 })?;
        }

        Terminal::show_cursor()?;
        Terminal::commit()?;

        Ok(())
    }

    fn eval_event(&mut self, event: &Event) {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers,
            ..
        }) = event
            && *modifiers == KeyModifiers::CONTROL
        {
            self.should_quit = true;
        }
    }

    fn draw_rows() -> Result<(), io::Error> {
        let TerminalSize { col, row } = Terminal::size()?;
        Terminal::begin()?;

        for r in 0..row {
            Terminal::clear_line()?;
            Terminal::print("~")?;

            if r == row / 4 {
                Editor::greet()?;
            }

            if r < col - 1 {
                Terminal::print("\r\n")?;
            }
        }

        Terminal::commit()?;

        Ok(())
    }

    fn greet() -> Result<(), io::Error> {
        let TerminalSize { col, .. } = Terminal::size()?;
        let message = format!("hello from cord version: {VERSION}");

        let mid = (col / 2) as usize;
        let start = mid - message.len() / 2;
        let left_pad = " ".repeat(start);
        Terminal::print(&format!("{left_pad}{message}"))?;

        Ok(())
    }
}
