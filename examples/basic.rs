// extern crate line_editor;

// use std::io;

// use crossterm::{terminal::{enable_raw_mode, disable_raw_mode}, style::Stylize};
// use line_editor::{LineEditor, Token, Signal, LineEditorState};

// fn main() -> io::Result<()>  {
//     enable_raw_mode()?;
//     let mut rl = LineEditor::new();

//     loop {
//         match rl.read_token() {
//             Token::Word(word) => {
//                 match rl.state {
//                     LineEditorState::NewWord          => print!("{}", word.yellow()),
//                     LineEditorState::Comment          => print!("{}", word.green()),
//                     LineEditorState::Annotation       => print!("{}", word.blue()),
//                     LineEditorState::StringLiteral    => print!("{}", word.dark_yellow()),
//                     LineEditorState::EscapedChar      => print!("{}", word.yellow()),
//                     LineEditorState::ObjectReading(_) => print!("{}", word.yellow()),
//                 }
//             },
//             Token::Signal(sig) => match sig {
//                 Signal::Interrupt => {
//                     println!("\nKeyboard Interrupt"); // LOG
//                     break;
//                 },
//                 Signal::Tab => todo!(),
//                 Signal::Enter => todo!(),
//                 Signal::Up => todo!(),
//                 Signal::Down => todo!(),
//                 Signal::Left => todo!(),
//                 Signal::Right => todo!(),
//             },
//             Token::Empty => todo!(),
//         }
//     }
//     disable_raw_mode()
// }

fn main() {
    // 
}