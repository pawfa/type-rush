use std::str::FromStr;
use std::fmt::{Error, Display, Formatter};
use core::fmt;
use std::error;

#[derive(Debug, Clone)]
pub struct TokenError;

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed matching")
    }
}

impl error::Error for TokenError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
