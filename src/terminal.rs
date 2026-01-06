use crossterm::{
    QueueableCommand, cursor, style,
    terminal::{self, ClearType},
};
use std::io::{self, Write};

pub struct TerminalSize {
    pub row: u16,
    pub col: u16,
}

#[derive(Debug, Default)]
pub struct CursorPos {
    pub row: u16,
    pub col: u16,
}

pub struct Terminal {}

impl Terminal {
    /// # Errors
    pub fn init() -> io::Result<()> {
        terminal::enable_raw_mode()?;
        Terminal::clear_screen()?;
        Terminal::move_cursor(&CursorPos { row: 0, col: 0 })?;

        Terminal::commit()?;

        Ok(())
    }

    /// # Errors
    pub fn close() -> io::Result<()> {
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// # Errors
    pub fn begin() -> io::Result<()> {
        Ok(())
    }

    /// # Errors
    pub fn commit() -> io::Result<()> {
        io::stdout().flush()
    }

    /// # Errors
    pub fn clear(t: ClearType) -> io::Result<()> {
        io::stdout().queue(terminal::Clear(t))?;
        Ok(())
    }

    /// # Errors
    pub fn clear_line() -> io::Result<()> {
        io::stdout().queue(terminal::Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    /// # Errors
    pub fn clear_screen() -> io::Result<()> {
        io::stdout().queue(terminal::Clear(ClearType::All))?;
        Ok(())
    }

    /// # Errors
    pub fn move_cursor(pos: &CursorPos) -> io::Result<()> {
        io::stdout().queue(cursor::MoveTo(pos.col, pos.row))?;
        Ok(())
    }

    /// # Errors
    pub fn hide_cursor() -> io::Result<()> {
        io::stdout().queue(cursor::Hide)?;
        Ok(())
    }

    /// # Errors
    pub fn show_cursor() -> io::Result<()> {
        io::stdout().queue(cursor::Show)?;
        Ok(())
    }

    /// # Errors
    pub fn size() -> io::Result<TerminalSize> {
        let (col, row) = terminal::size()?;
        Ok(TerminalSize { row, col })
    }

    /// # Errors
    pub fn print(s: &str) -> io::Result<()> {
        io::stdout().queue(style::Print(s))?;
        Ok(())
    }
}
