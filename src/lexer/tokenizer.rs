use std::iter::Peekable;
use std::str::{Chars, FromStr};
use crate::lexer::tokens::kind::TokenKind;
use crate::lexer::tokens::parenthesis::Parenthesis;
use crate::lexer::tokens::punctuator::Punctuator;
use crate::lexer::tokens::double_comparison::DoubleComparison;
use crate::lexer::tokens::single_comparison::SingleComparison;
use crate::lexer::tokens::triple_comparison::TripleComparison;
use crate::lexer::token::{Token, TokenizerError};
use crate::lexer::tokens::assignment::Assignment;
use crate::lexer::tokens::arithmetic_operator::ArithmeticOperator;

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

            match buf_char {
                _ if SingleComparison::from_char(buf_char).is_ok()  => {
                    let mut s = String::new();
                    s.push(buf_char);
                    self.column_number += 1;
                    let second = self.preview_next();
                    match second {
                        None => self.tokens.push(Token::new(TokenKind::SingleComparison(buf_char), self.line_number, self.column_number)),
                        Some(key) => {
                            s.push(key);
                                if let Ok(double_comparison) = DoubleComparison::from_str(&s) {
                                    self.tokens.push(Token::new(TokenKind::DoubleComparison(double_comparison), self.line_number, self.column_number));
                                    self.column_number += 1;
                                } else {
                                    let third = self.preview_next();
                                    match third {
                                        None => return Err(TokenizerError::new("Second sign comparison token error")),
                                        Some(key) => {
                                            s.push(key);
                                            if let Ok(triple_comparison) = TripleComparison::from_str(&s) {
                                                self.tokens.push(Token::new(TokenKind::TripleComparison(triple_comparison), self.line_number, self.column_number));
                                                self.column_number += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                _ if Assignment::from_char(buf_char).is_ok()  => {
                    self.column_number += 1;
                    self.tokens.push(Token::new(TokenKind::Assignment(buf_char), self.line_number, self.column_number))
                }
                _ if ArithmeticOperator::from_char(buf_char).is_ok()  => {
                    self.column_number += 1;
                    self.tokens.push(Token::new(TokenKind::ArithmeticOperator(buf_char), self.line_number, self.column_number))
                }
                _ if Punctuator::from_char(buf_char).is_ok()  => {
                    self.column_number += 1;
                    self.tokens.push(Token::new(TokenKind::Punctuator(buf_char), self.line_number, self.column_number))
                }
                _ if Parenthesis::from_char(buf_char).is_ok()  => {
                    self.column_number += 1;
                    let par = Parenthesis::from_char(buf_char).unwrap();
                    self.tokens.push(Token::new(TokenKind::Parenthesis(par), self.line_number, self.column_number))
                }
                _ if buf_char.is_alphabetic() || buf_char.is_alphanumeric() => {
                    let mut s = String::new();
                    s.push(buf_char);
                    self.column_number += 1;
                    loop {
                        let key = self.preview_next().unwrap();
                        if key.is_alphabetic() {
                            s.push(self.next()?);
                            self.column_number += 1;
                        } else {
                            break;
                        }
                    }

                    let buf_compare: &str = &s;
                    if let Ok(keyword) = FromStr::from_str(buf_compare) {
                        self.tokens.push(Token::new(TokenKind::Keyword(keyword), self.line_number, self.column_number))
                    } else {
                        self.tokens.push(Token::new(TokenKind::Identifier(s), self.line_number, self.column_number))
                    }
                }
                '\u{0020}' | '\u{0009}' | '\u{000B}' | '\u{000C}' | '\u{00A0}' | '\u{FEFF}' |
                '\u{1680}' | '\u{2000}'..='\u{200A}' | '\u{202F}' | '\u{205F}' | '\u{3000}' => (),
                '\u{000A}' => {
                    self.line_number += 1;
                }
                '\u{000D}' => (),
                _ => {
                    self.tokens.push(Token::new(TokenKind::Illegal(buf_char), self.line_number, self.column_number))
                }
            }

            if self.preview_next().is_none() {
                return Err(TokenizerError::new("finished"));
            }
        }

    }
}
