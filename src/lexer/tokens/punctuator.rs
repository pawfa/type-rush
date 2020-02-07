use std::str::FromStr;
use std::fmt::{Error, Display, Formatter};

pub enum Punctuator {
    Comma,
    Semicolon,
    Colon,
    QuestionMark,
    Dot,
}

impl Punctuator {
    pub fn to_str(&self) -> &str {
        match self {
            &Punctuator::Dot => ".",
            &Punctuator::Comma => ",",
            &Punctuator::Semicolon => ";",
            &Punctuator::Colon => ":",
            &Punctuator::QuestionMark => "?",
        }
    }
    pub fn from_char(s: char) -> Result<Self,()> {
        match s {
            '.' => Ok(Punctuator::Dot),
            ',' => Ok(Punctuator::Comma),
            ';' => Ok(Punctuator::Semicolon),
            ':' => Ok(Punctuator::Colon),
            '?' => Ok(Punctuator::QuestionMark),
            _ => Err(()),
        }
    }
}

impl Display for Punctuator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "invalid token")
    }
}

pub struct PunctuatorError;
impl Display for PunctuatorError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "invalid token")
    }
}

impl FromStr for Punctuator {
    type Err = PunctuatorError;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        match s {
            "." => Ok(Punctuator::Dot),
            "," => Ok(Punctuator::Comma),
            ";" => Ok(Punctuator::Semicolon),
            ":" => Ok(Punctuator::Colon),
            "?" => Ok(Punctuator::QuestionMark),
            _ => Err(PunctuatorError),
        }
    }
}