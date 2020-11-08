use core::fmt;
use std::error;

#[derive(Debug)]
pub enum RunError {
    Message(&'static str)
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RunError::Message(v) =>
                write!(f, "{} message", v),
        }
    }
}

impl error::Error for RunError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            RunError::Message(_v) => None,
        }
    }
}