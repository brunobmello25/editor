use std::io;

use crossterm::{cursor, execute, terminal};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), io::Error> {
        terminal::enable_raw_mode()?;
        Self::clear_screen()
    }

    pub fn clear_screen() -> Result<(), io::Error> {
        let mut stdout = io::stdout();
        execute!(stdout, terminal::Clear(terminal::ClearType::All))
    }

    pub fn terminate() -> Result<(), io::Error> {
        terminal::disable_raw_mode()
    }

    pub fn size() -> Result<(u16, u16), io::Error> {
        terminal::size()
    }

    pub fn move_cursor(x: u16, y: u16) -> Result<(), io::Error> {
        let mut stdout = io::stdout();
        execute!(stdout, cursor::MoveTo(x, y))
    }
}
