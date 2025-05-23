use std::io;

use crate::terminal::Terminal;
use crossterm::event::{
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers, read,
};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn draw_rows() -> Result<(), io::Error> {
        let height = Terminal::size()?.1;
        for current_row in 0..height {
            Terminal::print("~")?;
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn repl(&mut self) -> Result<(), io::Error> {
        loop {
            self.update()?;
            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if modifiers == &KeyModifiers::CONTROL => self.should_quit = true,
                _ => (),
            }
        }
    }

    fn update(&self) -> Result<(), io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
            return Ok(());
        }

        Terminal::hide_cursor()?;
        Self::draw_rows()?;
        Terminal::move_cursor(0, 0)?;
        Terminal::show_cursor()
    }
}
