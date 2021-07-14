use std::fmt::{Error, Display, Formatter};
use crate::lexer::token_kinds::token_error::TokenError;

#[derive(Clone, Copy, PartialEq)]
pub enum TripleComparison {
    StrictEqual,
    StrictNotEqual,
}

impl TripleComparison {
    pub fn to_str(&self) -> &str {
        match self {
            &TripleComparison::StrictEqual => "===",
            &TripleComparison::StrictNotEqual => "!==",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, TokenError> {
        match s {
            "===" => Ok(TripleComparison::StrictEqual),
            "!==" => Ok(TripleComparison::StrictNotEqual),
            _ => Err(TokenError),
        }
    }
}

impl Display for TripleComparison {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &TripleComparison::StrictEqual => write!(f, "==="),
            &TripleComparison::StrictNotEqual => write!(f, "!=="),
        }

    }
}
