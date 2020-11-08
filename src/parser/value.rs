use std::fmt::{Display, Formatter};
use core::fmt;

#[derive(Clone, PartialEq)]
pub enum PrimitiveValue {
    Boolean(bool),
    String(String),
    Num(f64),
}

impl Display for PrimitiveValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            PrimitiveValue::Boolean(v) => v.fmt(f),
            PrimitiveValue::String(v) => v.fmt(f),
            PrimitiveValue::Num(x) => write!(f, "{}", x),
        }
    }
}