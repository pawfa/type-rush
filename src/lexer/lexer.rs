use std::iter::Peekable;
use std::str::{Chars, FromStr};
use crate::lexer::tokens::kind::TokenKind;
use crate::lexer::tokens::parenthesis::Parenthesis;
use crate::lexer::tokens::punctuator::Punctuator;
use crate::lexer::tokens::double_comparison::DoubleComparison;
use crate::lexer::tokens::single_comparison::SingleComparison;
use crate::lexer::tokens::triple_comparison::TripleComparison;
use crate::lexer::token::Token;
use crate::lexer::token_errors::TokenizerError;
use crate::lexer::tokens::assignment::Assignment;
use crate::lexer::tokens::arithmetic_operator::ArithmeticOperator;
use crate::lexer::tokens::literal::Literal;


pub struct Lexer<'a> {
    tokens: Vec<Token>,
    line_number: u64,
    column_number: u64,
    stream: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(stream: &'a String) -> Lexer<'a> {
        Lexer {
            tokens: Vec::new(),
            line_number: 1,
            column_number: 1,
            stream: stream.chars().peekable(),
        }
    }

    fn preview_next(&mut self) -> Option<char> {
        self.stream.peek().copied()
    }

    fn next(&mut self) -> Result<char, TokenizerError> {
        match self.stream.next() {
            Some(ch) => Ok(ch),
            None => Err(TokenizerError::NoTokens),
        }
    }

    pub fn analyse(&mut self) -> Result<Vec<Token>, TokenizerError> {
        loop {
            let current_char = self.next()?;

            match current_char {
                _ if SingleComparison::from_char(current_char).is_ok()  => {
                    let mut s = String::new();
                    s.push(current_char);
                    self.column_number += 1;
                    let second = self.preview_next();
                    match second {
                        None => self.tokens.push(Token::new(TokenKind::SingleComparison(current_char), self.line_number, self.column_number)),
                        Some(key) => {
                            s.push(key);
                                if let Ok(double_comparison) = DoubleComparison::from_str(&s) {
                                    self.tokens.push(Token::new(TokenKind::DoubleComparison(double_comparison), self.line_number, self.column_number));
                                    self.column_number += 1;
                                } else {
                                    let third = self.preview_next();
                                    match third {
                                        None => return Err(TokenizerError::Message("Second sign comparison token error")),
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
                _ if Assignment::from_char(current_char).is_ok()  => {
                    self.column_number += 1;
                    self.tokens.push(Token::new(TokenKind::Assignment(current_char), self.line_number, self.column_number))
                }
                _ if ArithmeticOperator::from_char(current_char).is_ok()  => {
                    self.column_number += 1;
                    let operator = ArithmeticOperator::from_char(current_char).unwrap();
                    self.tokens.push(Token::new(TokenKind::ArithmeticOperator(operator), self.line_number, self.column_number))
                }
                _ if Punctuator::from_char(current_char).is_ok()  => {
                    self.column_number += 1;
                    self.tokens.push(Token::new(TokenKind::Punctuator(current_char), self.line_number, self.column_number))
                }
                _ if Parenthesis::from_char(current_char).is_ok()  => {
                    self.column_number += 1;
                    let par = Parenthesis::from_char(current_char).unwrap();
                    self.tokens.push(Token::new(TokenKind::Parenthesis(par), self.line_number, self.column_number))
                }
                _ if current_char.is_alphabetic() || current_char.is_alphanumeric() => {
                    let mut s = String::new();
                    s.push(current_char);
                    self.column_number += 1;
                    loop {
                        let key = self.preview_next();

                        match key {
                            None => {}
                            Some(k) => {
                                if k.is_alphabetic() || k.is_alphanumeric() {

                                    s.push(self.next()?);
                                    self.column_number += 1;
                                } else {
                                    break;
                                }
                            }
                        }

                    }

                    let buf_compare: &str = &s;
                    if let Ok(keyword) = FromStr::from_str(buf_compare) {
                        self.tokens.push(Token::new(TokenKind::Keyword(keyword), self.line_number, self.column_number))
                    } else {
                        match s.parse() {
                            Ok(s) => self.tokens.push(Token::new(TokenKind::Literal(Literal::Num(s)), self.line_number, self.column_number)),
                            Err(_) => self.tokens.push(Token::new(TokenKind::Identifier(buf_compare.to_string()), self.line_number, self.column_number)),
                        }
                    }
                }
                //tabs, nbsp, spaces
                '\u{0009}' | '\u{000B}' | '\u{00A0}' | '\u{FEFF}' | '\u{202F}' | '\u{205F}' => (),
                // whitespace
                '\u{0020}' => {
                    self.column_number += 1;
                },
                //carriage return
                '\u{000D}' => {
                    self.line_number += 1;
                    self.column_number = 0;
                },
                any => return Err(TokenizerError::Illegal(any,self.line_number, self.column_number))
            }

            if self.preview_next().is_none() {
                break
            }
        }
        return Ok(self.tokens.clone())
    }
}

