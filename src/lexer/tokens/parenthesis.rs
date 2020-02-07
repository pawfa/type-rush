use std::str::FromStr;
use std::fmt::{Error, Display, Formatter};
use core::fmt;
use std::error;

pub struct ParenthesisError;

#[derive(Debug, Clone)]
pub struct CharError;

impl fmt::Display for CharError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for CharError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

pub enum Parenthesis {
    LBrace,
    RBrace,
    LParen,
    RParen,
}

impl Parenthesis {
    pub fn to_str(&self) -> &str {
        match self {
            &Parenthesis::LBrace => "{",
            &Parenthesis::RBrace => "}",
            &Parenthesis::LParen => "(",
            &Parenthesis::RParen => ")",
        }
    }

    pub fn from_char(s: char) -> Result<Self,CharError> {
        match s {
            '{' => Ok(Parenthesis::LBrace),
            '}' => Ok(Parenthesis::RBrace),
            '(' => Ok(Parenthesis::LParen),
            ')' => Ok(Parenthesis::RParen),
            _ => Err(CharError),
        }
    }
}

impl Display for Parenthesis {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "invalid token")
    }
}

impl Display for ParenthesisError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "invalid token")
    }
}

impl FromStr for Parenthesis {
    type Err = ParenthesisError;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        match s {
            "{" => Ok(Parenthesis::LBrace),
            "}" => Ok(Parenthesis::RBrace),
            "(" => Ok(Parenthesis::LParen),
            ")" => Ok(Parenthesis::RParen),
            _ => Err(ParenthesisError),
        }
    }
}