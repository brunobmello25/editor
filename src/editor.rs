use std::io;

use crate::terminal::Terminal;
use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};

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
        let size = Terminal::size()?;
        dbg!(size);

        for _ in 0..size.1 {
            for col in 0..size.0 {
                if col == 0 {
                    print!("~");
                } else {
                    print!(" ");
                }
            }
            print!("\r\n");
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
            print!("Goodbye.\r\n");
            return Ok(());
        }

        Self::draw_rows()?;
        Terminal::move_cursor(0, 0)?;

        Ok(())
    }
}
