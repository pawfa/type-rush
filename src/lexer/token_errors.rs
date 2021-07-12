use core::fmt;

#[derive(Debug)]
pub enum TokenizerError {
    Illegal(char, u64, u64),
    Message(&'static str),
    NoTokens,
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenizerError::Illegal(v, m, t) =>
                write!(f, "Illegal character {} \nline: {}\ncolumn: {}", v, m, t),
            TokenizerError::NoTokens =>
                write!(f, "There are no Tokens left to analyse"),
            TokenizerError::Message(v) =>
                write!(f, "{}", v),
        }
    }
}
