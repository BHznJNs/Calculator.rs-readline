mod cursor;
mod text_type;

use std::{io::{self, Stdout, Write}, fmt::Display};

use crossterm::{event::{self,
    Event, KeyEvent,
    KeyEventKind,
}, style::Stylize};

pub use cursor::Cursor;
pub use text_type::TextType;

use crate::terminal::text_type::match_tx_type;

pub struct Terminal {
    pub stdout: Stdout,
    pub cursor: Cursor,
}

const BACKSPACE: &'static str = "\x1B[K";

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            stdout: io::stdout(),
            cursor: Cursor::new(),
        }
    }

    pub fn get_key(&self) -> Option<KeyEvent> {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                return Some(key)
            }
        }
        None
    }

    pub fn cursor_col(&self) -> io::Result<usize> {
        Ok(self.cursor.position()? as usize)
    }

    // --- --- --- --- --- ---

    // print char, &str, String
    pub fn print<T: Stylize>(&mut self, text: T, text_type: TextType)
        where <T as Stylize>::Styled: Display
    {
        let colored_text =
            match_tx_type(text, text_type);
        print!("{}", colored_text);
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    pub fn back(&mut self, cell: u16) -> io::Result<()> {
        for _ in 0..cell {
            self.cursor.left(1)?;
            self.clear_after_cursor()?;
            self.stdout.flush()?;
        }
        Ok(())
    }

    pub fn new_line(&mut self) {
        println!();
    }

    pub fn clear_after_cursor(&mut self) -> io::Result<()> {
        print!("{}", BACKSPACE);
        self.flush()
    }
}