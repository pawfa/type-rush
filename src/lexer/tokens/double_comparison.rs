use std::str::FromStr;
use std::fmt::{Error, Display, Formatter};

pub enum DoubleComparison {
    GreaterOrEqual,
    LesserOrEqual,
    Equal,
    NotEqual,
}

impl DoubleComparison {
    pub fn to_str(&self) -> &str {
        match self {
            &DoubleComparison::GreaterOrEqual => ">=",
            &DoubleComparison::LesserOrEqual => "<=",
            &DoubleComparison::Equal => "==",
            &DoubleComparison::NotEqual => "!=",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            ">=" => Ok(DoubleComparison::GreaterOrEqual),
            "<=" => Ok(DoubleComparison::LesserOrEqual),
            "==" => Ok(DoubleComparison::Equal),
            "!=" => Ok(DoubleComparison::NotEqual),
            _ => Err(()),
        }
    }
}

impl Display for DoubleComparison {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "invalid token")
    }
}