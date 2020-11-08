use std::fmt::{Display, Formatter};
use core::fmt;
use crate::engine::expressions::function::Function;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, PartialEq)]
pub enum JSValue {
    Boolean(bool),
    String(String),
    Num(f64),
    Function(Function),
    Undefined
}

impl Display for JSValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            JSValue::Boolean(v) => v.fmt(f),
            JSValue::String(v) => v.fmt(f),
            JSValue::Num(x) => write!(f, "{}", x),
            JSValue::Function(x) => write!(f, "{}", x),
            JSValue::Undefined => write!(f, "{}", "undefined"),
        }
    }
}

impl Add for JSValue {
    type Output = Self;

    fn add(self, second: Self) -> Self {
        match (self, second) {
            (JSValue::Num(v), JSValue::Num(z)) => JSValue::Num(v+z),
            _ => JSValue::Undefined
        }
    }
}

impl Sub for JSValue {
    type Output = Self;

    fn sub(self, second: Self) -> Self {
        match (self, second) {
            (JSValue::Num(v), JSValue::Num(z)) => JSValue::Num(v-z),
            _ => JSValue::Undefined
        }
    }
}

impl Mul for JSValue {
    type Output = Self;

    fn mul(self, second: Self) -> Self {
        match (self, second) {
            (JSValue::Num(v), JSValue::Num(z)) => JSValue::Num(v*z),
            _ => JSValue::Undefined
        }
    }
}

impl Div for JSValue {
    type Output = Self;

    fn div(self, second: Self) -> Self {
        match (self, second) {
            (JSValue::Num(v), JSValue::Num(z)) => JSValue::Num(v/z),
            _ => JSValue::Undefined
        }
    }
}