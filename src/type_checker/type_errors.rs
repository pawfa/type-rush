use core::fmt;
use std::error;

#[derive(Debug)]
pub enum TypeError {
    Message(String)
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeError::Message(v) =>
                write!(f, "Type checking error: \n{}", v),
        }
    }
}