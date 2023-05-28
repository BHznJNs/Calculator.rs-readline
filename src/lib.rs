// mod token;
// mod signal;
// mod state;
// mod ansi_escapes;

// pub use token::Token;
// pub use signal::Signal;
// pub use state::LineEditorState;

// use std::io::{Stdout, self, Write};

// use crossterm::{event::{self,
//     Event, KeyCode, KeyEvent,
//     KeyEventKind, KeyModifiers
// }, cursor, execute};

// pub struct LineEditor {
//     pub state: LineEditorState,
//     current_line: String,
//     last_word: String,

//     stdout: Stdout,
// }

// impl LineEditor {
//     pub fn new() -> Self {
//         LineEditor {
//             state: LineEditorState::NewWord,
//             current_line: String::new(),
//             last_word: String::new(),
//             stdout: io::stdout(),
//         }
//     }
//     fn get_key(&self) -> Option<KeyEvent> {
//         if let Ok(Event::Key(key)) = event::read() {
//             if key.kind == KeyEventKind::Press {
//                 return Some(key)
//             }
//         }
//         None
//     }

//     pub fn read_token(&mut self) -> Token {
//         let mut word = String::new();

//         let result =
//         loop {
//             let Some(key) = self.get_key() else {
//                 continue;
//             };
//             // ctrl + c -> Interrupt
//             if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
//                 break Token::Signal(Signal::Interrupt);
//             }

//             match key.code {
//                 KeyCode::Enter     => break Token::Signal(Signal::Enter),
//                 KeyCode::Left      => break Token::Signal(Signal::Left),
//                 KeyCode::Right     => break Token::Signal(Signal::Right),
//                 KeyCode::Up        => break Token::Signal(Signal::Up),
//                 KeyCode::Down      => break Token::Signal(Signal::Down),
//                 KeyCode::Tab       => break Token::Signal(Signal::Tab),
//                 KeyCode::Backspace => {
//                     execute!(self.stdout, cursor::MoveLeft(1)).unwrap();
//                     print!("{}", ansi_escapes::BACKSPACE);
//                     self.stdout.flush().unwrap();
//                 },

//                 KeyCode::Char(ch) => {
//                     cursor::MoveLeft(1);

//                     print!("{}", ch);
//                     self.stdout.flush().unwrap();

//                     if Token::is_divider(ch) {
//                         self.state_change(ch);
//                         break Token::Word(word);
//                     } else {
//                         // self.output_char(ch);
//                         word.push(ch)
//                     }
//                 },
//                 _ => {}
//             }
//         };
//         return result;
//     }

//     pub fn get_line(mut self) -> (Self, String) {
//         let line = self.current_line;
//         self.current_line = String::new();
//         (self, line)
//     }


//     fn state_change(&mut self, divider: char) {
//         self.state =
//         match divider {
//             '\\' => LineEditorState::EscapedChar,
//             '$'  => LineEditorState::Annotation,
//             '#'  => LineEditorState::Comment,
//             '.'  => LineEditorState::ObjectReading(self.last_word.clone()),
//             '\'' | '"'  => if self.state == LineEditorState::StringLiteral {
//                 LineEditorState::NewWord
//             } else {
//                 LineEditorState::StringLiteral
//             },

//             ','
//             | '('
//             | ')'
//             | '['
//             | ']'
//             | '{'
//             | '}'
//             | '+'
//             | '-'
//             | '*'
//             | '/'
//             | '^'
//             | '!'
//             | ' '  => LineEditorState::NewWord,
//             _ => {
//                 println!("Unexpected character: {}", divider);
//                 LineEditorState::NewWord
//             }
//         };
//     }

//     // fn output_char(&mut self, ch: char) {
//     //     match self.state {
//     //         LineEditorState::NewWord => print!("{}", ch.yellow()),
//     //         LineEditorState::Comment => print!("{}", ch.green()),
//     //         LineEditorState::Annotation => print!("{}", ch.blue()),
//     //         LineEditorState::StringLiteral => print!("{}", ch.dark_yellow()),
//     //         LineEditorState::EscapedChar => print!("{}", ch.yellow()),
//     //         LineEditorState::ObjectReading(_) => print!("{}", ch.yellow()),
//     //     }
//     //     self.stdout.flush().unwrap();
//     // }
// }