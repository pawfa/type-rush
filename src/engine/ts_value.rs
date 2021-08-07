use std::fmt::{Display, Formatter};
use core::fmt;
use crate::engine::expressions::function::Function;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, PartialEq)]
pub enum TSValue {
    Boolean(bool),
    String(String),
    Num(f64),
    Function(Function),
    Undefined
}

impl Display for TSValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TSValue::Boolean(v) => v.fmt(f),
            TSValue::String(v) => v.fmt(f),
            TSValue::Num(x) => write!(f, "{}", x),
            TSValue::Function(x) => write!(f, "{}", x),
            TSValue::Undefined => write!(f, "{}", "undefined"),
        }
    }
}

impl Add for TSValue {
    type Output = Self;

    fn add(self, second: Self) -> Self {
        match (self, second) {
            (TSValue::Num(v), TSValue::Num(z)) => TSValue::Num(v+z),
            _ => TSValue::Undefined
        }
    }
}

impl Sub for TSValue {
    type Output = Self;

    fn sub(self, second: Self) -> Self {
        match (self, second) {
            (TSValue::Num(v), TSValue::Num(z)) => TSValue::Num(v-z),
            _ => TSValue::Undefined
        }
    }
}

impl Mul for TSValue {
    type Output = Self;

    fn mul(self, second: Self) -> Self {
        match (self, second) {
            (TSValue::Num(v), TSValue::Num(z)) => TSValue::Num(v*z),
            _ => TSValue::Undefined
        }
    }
}

impl Div for TSValue {
    type Output = Self;

    fn div(self, second: Self) -> Self {
        match (self, second) {
            (TSValue::Num(v), TSValue::Num(z)) => TSValue::Num(v/z),
            _ => TSValue::Undefined
        }
    }
}