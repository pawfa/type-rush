use std::iter::Peekable;
use std::str::{Chars, FromStr};
use crate::lexer::tokens::kind::TokenKind;
use std::error;
use core::fmt;
use crate::lexer::token::{Token, TokenizerError};

pub struct Tokenizer<'a> {
    pub tokens: Vec<Token>,
    line_number: u64,
    column_number: u64,
    buffer: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(buffer: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            tokens: Vec::new(),
            line_number: 1,
            column_number: 0,
            buffer: buffer.chars().peekable(),
        }
    }

    fn preview_next(&mut self) -> Option<char> {
        self.buffer.peek().copied()
    }

    fn next(&mut self) -> Result<char, TokenizerError> {
        match self.buffer.next() {
            Some(ch) => Ok(ch),
            None => Err(TokenizerError::new("finished")),
        }
    }

    pub fn lex(&mut self) -> Result<(), TokenizerError> {
        loop {
            let buf_char = self.next()?;
            println!("{}",buf_char);
            if self.preview_next().is_none() {
                return Err(TokenizerError::new("finished"));
            }
            match buf_char {
                _ if buf_char.is_alphabetic() || buf_char.is_alphanumeric() => {
                    let mut s = String::new();
                    s.push(buf_char);
                    loop {
                        let key = self.preview_next().unwrap();
                        if key.is_alphabetic() {
                            s.push(self.next()?);
                        } else {
                            break;
                        }
                    }

                    println!("{}", s);
                    let buf_compare: &str = &s;
                    if let Ok(keyword) = FromStr::from_str(buf_compare) {
                        self.tokens.push(Token::new(TokenKind::Keyword(keyword)))
                    } else {
                        self.tokens.push(Token::new(TokenKind::Identifier(s)))
                    }
                }
                _ => self.tokens.push(Token::new(TokenKind::Illegal(buf_char)))
            }
        }

    }
}

