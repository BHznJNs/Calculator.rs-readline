mod token;
mod signal;
mod candidate;

use std::io;

use crossterm::{event::{KeyModifiers, KeyCode}, cursor};
use crate::{terminal::{
        Terminal, TextType,
    }, history::History,
    completer::Completer,
    line_editor::token::{Token, Tokens},
};
pub use signal::Signal;
use candidate::Candidate;

pub struct LineEditor {
    terminal: Terminal,
    // history: History,
    candidate: Candidate,

    line_tokens: Tokens,
}

const PROMPT: &'static str = ">> ";

impl LineEditor {
    pub fn new() -> Self {
        LineEditor {
            terminal: Terminal::new(),
            // history: History::new(),
            candidate: Candidate::new(),

            line_tokens: Tokens::new(),
        }
    }

    fn is_at_right_end(&self, line: &String) -> io::Result<bool> {
        Ok(self.terminal.cursor_col()? == line.len())
    }

    fn display_hint(&mut self) -> io::Result<()> {
        if let Some(hint) = self.candidate.next() {
            self.terminal.cursor.hide()?;
            self.terminal.clear_after_cursor()?;

            self.terminal.print(hint, TextType::Hint);
            self.terminal.flush()?;

            self.terminal.cursor.left(hint.len() as u16)?;
            self.terminal.cursor.show()?;
        }
        Ok(())
    }

    fn display_ch(&mut self, ch: char, state: TextType) -> io::Result<()> {
        self.line_tokens.append(ch, state);
        self.terminal.print(ch, state);
        self.terminal.flush()
    }
    fn display_keyword(&mut self, word: &str) -> io::Result<()> {
        self.terminal.cursor.hide()?;

        self.candidate.set(vec![]);
        self.terminal.cursor.left((word.len() - 1) as u16)?;
        self.terminal.print(word, TextType::Keyword);
        self.terminal.flush()?;

        self.terminal.cursor.show()
    }
    fn complete(&mut self, word: &mut String, line: &mut String) -> io::Result<()> {
        if let Some(hint) = self.candidate.current_hint() {
            *word += hint;
            *line += hint;
            self.line_tokens.append_partial(hint, TextType::Variable);
            self.terminal.print(hint, TextType::Variable);
            self.terminal.flush()?;
        }
        Ok(())
    }

    fn insert_edit(&mut self, ch: char, line: &mut String) -> io::Result<()> {
        let insert_pos = self.terminal.cursor_col()?;
        line.insert(insert_pos, ch);

        self.terminal.clear_after_cursor()?;
        self.terminal.cursor.save_pos()?;

        self.line_tokens.insert(&mut self.terminal, insert_pos, ch)?;

        self.terminal.cursor.restore_pos()?;
        self.terminal.cursor.right(1)?;
        Ok(())
    }

    pub fn read(&mut self, completer: &mut Completer) -> io::Result<Signal> {
        let mut line = String::new();
        let mut word = String::new();
        let mut state = TextType::Variable;

        print!("{}", PROMPT);
        self.terminal.flush()?;

        let result =
        loop {
            let Some(key) = self.terminal.get_key() else {
                continue;
            };

            // ctrl + c -> Interrupt
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                println!("\nKeyboard Interrupt");
                break Signal::Interrupt;
            }

            let is_at_right_end = self.is_at_right_end(&line)?;

            match key.code {
                KeyCode::Left  => self.terminal.cursor.left(1)?,
                KeyCode::Right => {
                    if is_at_right_end {
                        self.complete(&mut word, &mut line)?;
                        self.candidate.set(completer.complete(&word));
                    } else {
                        self.terminal.cursor.right(1)?
                    }
                },
                // use with history
                // KeyCode::Up    => todo!(),
                // KeyCode::Down  => todo!(),

                KeyCode::Enter => {
                    word.clear();
                    println!("Tokens: {:#?}", self.line_tokens.content); // LOG
                    self.line_tokens.clear();
                    self.terminal.new_line();
                    break Signal::NewLine(line);
                },
                KeyCode::Tab => {
                    if is_at_right_end {
                        self.display_hint()?;
                    }
                    continue;
                },
                KeyCode::Backspace => {
                    if is_at_right_end {
                        word.pop();
                        line.pop();
                        self.line_tokens.pop();
                        if !word.is_empty() {
                            self.candidate.set(completer.complete(&word));
                        }
                    }

                    self.terminal.back(1)?;
                },

                KeyCode::Char(ch) => {
                    if !is_at_right_end {
                        self.insert_edit(ch, &mut line)?;
                        continue;
                    }

                    // following is directly push
                    line.push(ch);

                    // in comment
                    if state == TextType::Comment {
                        self.display_ch(ch, state)?;
                        continue;
                    }

                    // if in String
                    if state == TextType::StringLiteral && ch != '\'' && ch != '"' {
                        self.display_ch(ch, state)?;
                        continue;
                    }

                    // if first char of a word is number,
                    // this word is NumberLiteral
                    if word.is_empty() && ch.is_ascii_digit() {
                        state = TextType::NumberLiteral;
                    }

                    if Token::is_divider(ch) {
                        match ch {
                            '#' => state = TextType::Comment,
                            '.' => {
                                if state == TextType::NumberLiteral {
                                    self.display_ch(ch, state)?;
                                    continue;
                                }
                            },
                            '"' | '\'' => {
                                state = if state == TextType::StringLiteral {
                                    TextType::Variable
                                } else {
                                    TextType::StringLiteral
                                };
                                self.display_ch(ch, TextType::StringLiteral)?;
                                continue;
                            },
                            _ => state = TextType::Didider,
                        }

                        self.candidate.set(vec![]);
                        self.display_ch(ch, state)?;
                        if state == TextType::Didider {
                            state = TextType::Variable;
                        }
                        word = String::new();
                    } else {
                        if let Some(hint) = self.candidate.current_hint() {
                            let first_hint_char = hint.chars().nth(0);
                            if first_hint_char != Some(ch) {
                                self.terminal.clear_after_cursor()?;
                            }
                        }

                        word.push(ch);
                        if Token::is_keyword(&word) {
                            // if current word is keyword
                            self.display_keyword(&word)?;
                            self.line_tokens.append_partial(&word, TextType::Keyword);
                            word = String::new();
                        } else {
                            // variable name
                            self.candidate.set(completer.complete(&word));
                            self.display_ch(ch, state)?;
                        }
                    }
                },
                _ => {}
            }
        };
        Ok(result)
    }
}