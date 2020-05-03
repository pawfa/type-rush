use std::str::FromStr;
use std::fmt::{Error, Display, Formatter};

#[derive(Clone, PartialEq)]
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

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {

        match self {
            &Keyword::Var => write!(f,"var"),
            &Keyword::Let => write!(f,"let"),
            &Keyword::Const => write!(f,"const"),
            &Keyword::Export => write!(f,"export"),
            &Keyword::Function => write!(f,"function"),
            &Keyword::Interface => write!(f,"interface"),
            &Keyword::Return => write!(f,"return"),
        }
    }
}

pub struct KeywordError;
impl Display for KeywordError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Keyword lexing error")
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