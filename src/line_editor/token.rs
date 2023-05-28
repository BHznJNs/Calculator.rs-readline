use std::io;

use crate::terminal::{TextType, Terminal};

const DIVIDER_MAP: [bool; 94] = [
    true,  // ' '
    true,  // '!'
    true,  // '"'
    true,  // '#'
    true,  // '$'
    false,
    false,
    true,  // '\''
    true,  // '('
    true,  // ')'
    true,  // '*'
    true,  // '+'
    true,  // ','
    true,  // '-'
    true,  // '.'
    true,  // '/'
    false, // '0'
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false, // '9'
    false,
    true,  // ';'
    true,  // '<'
    true,  // '='
    true,  // '>'
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    true,  // '['
    true,  // '\'
    true,  // ']'
    true,  // '^'
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    true,  // '{'
    false,
    true,  // '}'
];

const KEYWORD_LIST: [&'static str; 11] = [
    "out",
    "for",
    "if",
    "ctn",
    "brk",
    "import",
    "fn",
    "cl",
    "new",

    "true",
    "false"
];

// --- --- --- --- --- ---

#[derive(Debug)]
pub struct Token {
    pub type__: TextType,
    pub content: String,
}

impl Token {
    pub fn is_divider(ch: char) -> bool {
        const OFFSET: usize = 32; // offset to ASCII
        DIVIDER_MAP[(ch as usize) - OFFSET]
    }

    pub fn is_keyword(word: &str) -> bool {
        for keyword in KEYWORD_LIST {
            if word.eq(keyword) {
                return true;
            }
        }
        return false
    }
}

// --- --- --- --- --- ---

#[derive(Debug)]
pub struct Tokens {
    pub content: Vec<Token>
}

impl Tokens {
    pub fn new() -> Self {
        Tokens {
            content: vec![]
        }
    }

    pub fn append(&mut self, ch: char, type__: TextType) {
        if let Some(mut last_token) = self.content.pop() {
            if type__ == last_token.type__ && last_token.type__ != TextType::Didider {
                last_token.content.push(ch);
                self.content.push(last_token);
                return;
            }
            self.content.push(last_token);
        }
        self.content.push(Token {
            type__,
            content: ch.to_string(),
        })
    }
    pub fn append_partial(&mut self, word: &str, type__: TextType) {
        if let Some(mut token) = self.content.pop() {
            if type__ == TextType::Keyword {
                token.type__ = TextType::Keyword;
                token.content.clear();
            }
            token.content += word;
            self.content.push(token);
        }
    }
    pub fn pop(&mut self) {
        if let Some(mut token) = self.content.pop() {
            token.content.pop();
            if !token.content.is_empty() {
                // if this token has only one character,
                // remove it
                self.content.push(token);
            }
        }
    }

    pub fn insert(&mut self, terminal: &mut Terminal, mut index: usize, ch: char) -> io::Result<()> {
        let mut inserted_index = 0;

        // resolve inserted token
        for token in &mut self.content {
            if index <= token.content.len() && token.type__ != TextType::Didider {
                token.content.insert(index, ch);
                terminal.back(index as u16)?;
                break;
            }
            inserted_index += 1;
            index -= token.content.len();
        }

        // output tokens after inserted token
        for token in &self.content[inserted_index..] {
            terminal.print::<&str>(&token.content, token.type__);
        }
        terminal.flush()
    }

    pub fn clear(&mut self) {
        self.content.clear();
    }
}