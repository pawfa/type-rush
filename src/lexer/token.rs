use std::string::ToString;
use core::fmt;
use std::error;
use crate::lexer::tokens::kind::TokenKind;

pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self {
            kind,
        }
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