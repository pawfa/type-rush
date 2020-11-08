use std::fmt::{Display, Formatter};
use core::fmt;

#[derive(Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    String(String),
    Num(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Boolean(v) => v.fmt(f),
            Value::String(v) => v.fmt(f),
            Value::Num(x) => write!(f, "{}", x),
        }
    }
}