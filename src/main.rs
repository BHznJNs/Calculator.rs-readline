mod terminal;
mod completer;
mod history;
mod line_editor;

use std::io;
use completer::Completer;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crate::line_editor::{LineEditor, Signal};
fn main() -> io::Result<()> {
    let mut rl = LineEditor::new();
    let mut completer = Completer::new();

    completer.insert("ab");
    completer.insert("abc");
    completer.insert("abd");
    completer.insert("abe");
    completer.insert("abda");
    completer.insert("abea");

    enable_raw_mode()?;
    loop {
        match rl.read(&mut completer) {
            Ok(Signal::NewLine(line)) => {
                println!("Line: {line}")
            },
            Ok(Signal::Interrupt) => break,
            Err(_) => {
                println!("Readline Error!")
            }
        }
    }
    disable_raw_mode()
}