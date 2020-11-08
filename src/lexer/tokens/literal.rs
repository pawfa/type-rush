use std::fmt::{Display, Formatter};
use core::fmt;

#[derive(Clone, PartialEq)]
pub enum Literal {
    Boolean(bool),
    String(String),
    Num(f64),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Literal::Boolean(v) => v.fmt(f),
            Literal::String(v) => v.fmt(f),
            Literal::Num(x) => write!(f, "{}", x),
        }
    }
}