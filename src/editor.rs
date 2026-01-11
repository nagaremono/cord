use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, read},
    terminal::ClearType,
};
use std::{cmp::min, io};
use u16;

use crate::{
    terminal::{CursorPos, Terminal, TerminalSize},
    view::{Renderer, View},
};

#[derive(Debug, Default)]
pub struct Editor<T: Renderer = View> {
    should_quit: bool,
    position: Location,
    view: T,
}

#[derive(Debug, Default, Clone)]
struct Location {
    pub row: u16,
    pub col: u16,
}

impl Location {
    fn set_row(&mut self, r: u16) {
        self.row = r;
    }

    fn set_col(&mut self, c: u16) {
        self.col = c;
    }
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
            self.eval_event(&event)?;
        }

        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), io::Error> {
        Terminal::begin()?;
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear(ClearType::CurrentLine)?;
            Terminal::print("Goodbye...\r\n")?;
        } else {
            self.view.render()?;
            self.move_cursor(&self.position.clone()).unwrap();
        }

        Terminal::show_cursor()?;
        Terminal::commit()?;

        Ok(())
    }

    fn eval_event(&mut self, event: &Event) -> Result<(), io::Error> {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers,
                ..
            }) if *modifiers == KeyModifiers::CONTROL => self.should_quit = true,

            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Up | KeyCode::Down | KeyCode::Right | KeyCode::Left => {
                    self.handle_cursor_event(*code)?;
                }
                _ => (),
            },

            _ => (),
        }

        Ok(())
    }

    fn handle_cursor_event(&mut self, key_code: KeyCode) -> Result<(), io::Error> {
        let TerminalSize { row: r, col: c } = Terminal::size()?;
        let l = &mut self.position;
        let curr_row = l.row;
        let curr_col = l.col;

        match key_code {
            KeyCode::Up => l.set_row(curr_row.saturating_sub(1)),
            KeyCode::Down => l.set_row(min(curr_row.saturating_add(1), r)),
            KeyCode::Right => l.set_col(min(curr_col.saturating_add(1), c)),
            KeyCode::Left => l.set_col(curr_col.saturating_sub(1)),

            _ => (),
        }

        self.sync_cursor()?;

        Ok(())
    }

    fn move_cursor(&mut self, pos: &Location) -> Result<(), io::Error> {
        Terminal::move_cursor(&CursorPos {
            row: pos.row,
            col: pos.col,
        })?;
        self.position = pos.clone();
        Ok(())
    }

    fn sync_cursor(&mut self) -> Result<(), io::Error> {
        Terminal::move_cursor(&CursorPos {
            row: self.position.row,
            col: self.position.col,
        })?;
        Ok(())
    }
}
