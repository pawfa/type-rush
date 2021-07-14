use std::str::FromStr;
use std::fmt::{Error, Display, Formatter};

pub enum Punctuator {
    Comma,
    Semicolon,
    Colon,
    QuestionMark,
    Dot,
    EOL
}

impl Punctuator {
    pub fn to_str(&self) -> &str {
        match self {
            &Punctuator::Dot => ".",
            &Punctuator::Comma => ",",
            &Punctuator::Semicolon => ";",
            &Punctuator::Colon => ":",
            &Punctuator::QuestionMark => "?",
            &Punctuator::EOL => "EOL"
        }
    }
    pub fn from_char(s: char) -> Result<Self,()> {
        match s {
            '.' => Ok(Punctuator::Dot),
            ',' => Ok(Punctuator::Comma),
            ';' => Ok(Punctuator::Semicolon),
            ':' => Ok(Punctuator::Colon),
            '?' => Ok(Punctuator::QuestionMark),
            '\n' => Ok(Punctuator::EOL),
            _ => Err(()),
        }
    }
}

impl Display for Punctuator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {

        match self {
            &Punctuator::Dot => write!(f, "."),
            &Punctuator::Comma => write!(f, ","),
            &Punctuator::Semicolon => write!(f, ";"),
            &Punctuator::Colon => write!(f, ":"),
            &Punctuator::QuestionMark => write!(f, "?"),
            &Punctuator::EOL => write!(f, "EOL")
        }

    }
}

pub struct PunctuatorError;
impl Display for PunctuatorError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Punctuator lexing error")
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
            "\n" => Ok(Punctuator::EOL),
            _ => Err(PunctuatorError),
        }
    }
}