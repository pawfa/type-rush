use std::str::FromStr;
use std::fmt::{Error, Display, Formatter};
use crate::lexer::tokens::token_error::TokenError;

#[derive(Clone, Copy, PartialEq)]
pub enum ArithmeticOperator {
    PLUS,
    MINUS,
    MULTIPLICATION,
    DIVISION
}

impl ArithmeticOperator {
    pub fn to_str(&self) -> &str {
        match self {
            &ArithmeticOperator::PLUS => "+",
            &ArithmeticOperator::MINUS => "-",
            &ArithmeticOperator::MULTIPLICATION => "*",
            &ArithmeticOperator::DIVISION => "/",
        }
    }

    pub fn from_char(s: char) -> Result<Self,TokenError> {
        match s {
            '+' => Ok(ArithmeticOperator::PLUS),
            '-' => Ok(ArithmeticOperator::MINUS),
            '*' => Ok(ArithmeticOperator::MULTIPLICATION),
            '/' => Ok(ArithmeticOperator::DIVISION),
            _ => Err(TokenError),
        }
    }
}

impl Display for ArithmeticOperator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self)
    }
}

impl FromStr for ArithmeticOperator {
    type Err = TokenError;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        match s {
            "+" => Ok(ArithmeticOperator::PLUS),
            "-" => Ok(ArithmeticOperator::MINUS),
            "*" => Ok(ArithmeticOperator::MULTIPLICATION),
            "/" => Ok(ArithmeticOperator::DIVISION),
            _ => Err(TokenError),
        }
    }
}