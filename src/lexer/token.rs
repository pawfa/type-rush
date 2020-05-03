use std::string::ToString;
use core::fmt;
use std::error;
use crate::lexer::tokens::kind::TokenKind;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line_number: u64,
    pub column_number: u64,
}

impl Token {
    pub fn new(kind: TokenKind, line_number: u64, column_number: u64) -> Self {
        Self {
            kind,
            line_number,
            column_number
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug)]
pub struct TokenizerError {
    details: String,
}

impl TokenizerError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for TokenizerError {
    fn description(&self) -> &str {
        &self.details
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}