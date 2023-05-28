use std::io::{self, Stdout};

use crossterm::{execute, cursor};

pub struct Cursor {
    stdout: Stdout,
    prompt_len: u16,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            stdout: io::stdout(),
            prompt_len: 3,
        }
    }

    pub fn position(&self) -> io::Result<u16> {
        Ok(cursor::position()?.0 - self.prompt_len)
    }

    pub fn left(&mut self, cell: u16) -> io::Result<()> {
        if cursor::position()?.0 > self.prompt_len {
            execute!(self.stdout, cursor::MoveLeft(cell))?;
        }
        Ok(())
    }
    pub fn right(&mut self, cell: u16) -> io::Result<()> {
        execute!(self.stdout, cursor::MoveRight(cell))?;
        Ok(())
    }

    pub fn save_pos(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::SavePosition)?;
        Ok(())
    }
    pub fn restore_pos(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::RestorePosition)?;
        Ok(())
    }

    pub fn hide(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::Hide)?;
        Ok(())
    }
    pub fn show(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::Show)?;
        Ok(())
    }
}