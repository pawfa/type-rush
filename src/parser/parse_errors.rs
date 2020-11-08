use core::fmt;
use std::error;

#[derive(Debug)]
pub enum ParserError {
    GetToken,
    Generic,
    FunctionName,
    Message(&'static str)
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::GetToken =>
                write!(f, "Generic error - Failed getting token"),
            ParserError::Generic =>
                write!(f, ""),
            ParserError::FunctionName =>
                write!(f, "Function name is required"),
            ParserError::Message(v) =>
                write!(f, "{} message", v),
        }
    }
}

impl error::Error for ParserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ParserError::GetToken => None,
            ParserError::Generic => None,
            ParserError::FunctionName => None,
            ParserError::Message(_v) => None,
        }
    }
}