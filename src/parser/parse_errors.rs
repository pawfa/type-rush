use core::fmt;
use std::error;

#[derive(Debug)]
pub enum ParserError {
    GetToken,
    Generic,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::GetToken =>
                write!(f, "Failed getting token"),
            ParserError::Generic =>
                write!(f, "Failed getting token"),
        }
    }
}

impl error::Error for ParserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ParserError::GetToken => None,
            ParserError::Generic => None,
        }
    }
}