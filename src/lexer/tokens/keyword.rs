use std::str::FromStr;
use std::fmt::{Error, Display, Formatter};

pub enum Keyword {
    Var,
    Let,
    Const,
    Export,
    Function,
    Interface,
    Return,
}

impl Keyword {
    pub fn to_str(&self) -> & str {
        match self {
            &Keyword::Var => "var",
            &Keyword::Let => "let",
            &Keyword::Const => "const",
            &Keyword::Export => "export",
            &Keyword::Function => "function",
            &Keyword::Interface => "interface",
            &Keyword::Return => "return",
        }
    }
}

pub struct KeywordError;
impl Display for KeywordError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "invalid token")
    }
}

impl FromStr for Keyword {
    type Err = KeywordError;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        match s {
            "var" => Ok(Keyword::Var),
            "let" => Ok(Keyword::Let),
            "const" => Ok(Keyword::Const),
            "export" => Ok(Keyword::Export),
            "function" => Ok(Keyword::Function),
            "interface" => Ok(Keyword::Interface),
            "return" => Ok(Keyword::Return),
            _ => Err(KeywordError),
        }
    }
}