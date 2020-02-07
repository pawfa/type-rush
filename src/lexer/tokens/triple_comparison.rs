use std::str::FromStr;
use std::fmt::{Error, Display, Formatter};
use crate::lexer::tokens::token_error::TokenError;

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
        write!(f, "invalid token")
    }
}
