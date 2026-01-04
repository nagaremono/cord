use crossterm::{ExecutableCommand, cursor, terminal};
use std::io;

pub struct TerminalSize {
    pub row: u16,
    pub col: u16,
}

pub struct Terminal {}

impl Terminal {
    /// # Errors
    pub fn init() -> io::Result<()> {
        terminal::enable_raw_mode()?;
        Ok(())
    }

    /// # Errors
    pub fn close() -> io::Result<()> {
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// # Errors
    pub fn clear() -> io::Result<()> {
        io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    /// # Errors
    pub fn move_cursor(row: u16, col: u16) -> io::Result<()> {
        io::stdout().execute(cursor::MoveTo(row, col))?;
        Ok(())
    }

    /// # Errors
    pub fn size() -> io::Result<TerminalSize> {
        let (col, row) = crossterm::terminal::size()?;
        Ok(TerminalSize { row, col })
    }
}
