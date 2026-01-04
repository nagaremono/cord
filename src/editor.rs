use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use std::io;

use crate::terminal::{Terminal, TerminalSize};

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
    pub fn repl(&mut self) -> Result<(), std::io::Error> {
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
        if self.should_quit {
            Terminal::clear()?;
            print!("Goodbye...\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor(0, 0)?;
        }
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

        for r in 0..row {
            print!("~");
            if r < col - 1 {
                print!("\r\n");
            }
        }

        Ok(())
    }
}
