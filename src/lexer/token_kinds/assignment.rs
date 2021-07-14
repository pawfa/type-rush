use std::str::FromStr;
use std::fmt::{Error, Display, Formatter};
use crate::lexer::token_kinds::token_error::TokenError;

pub enum Assignment {
    Assign,
}

impl Assignment {
    pub fn to_str(&self) -> &str {
        match self {
            &Assignment::Assign => "=",
        }
    }

    pub fn from_char(s: char) -> Result<Self,TokenError> {
        match s {
            '=' => Ok(Assignment::Assign),
            _ => Err(TokenError),
        }
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self)
    }
}

impl FromStr for Assignment {
    type Err = TokenError;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        match s {
            "=" => Ok(Assignment::Assign),
            _ => Err(TokenError),
        }
    }
}