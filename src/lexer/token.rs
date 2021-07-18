use core::fmt;
use std::fmt::{Display, Formatter};
use crate::lexer::token_kinds::kind::TokenKind;

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
            column_number,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Token:\n{}Line number: {} \nColumn number: {}\n", self.kind, self.line_number, self.column_number)
    }
}
