use crate::parser::statement::Statement;
use std::fmt::{Display, Formatter};
use core::fmt;

#[derive(Clone, PartialEq)]
pub struct Function {
    pub expression: Box<Statement>,
    pub args: Vec<Statement>
}

impl Function {
    pub fn new(expression: Box<Statement>, args: Vec<Statement>) -> Self {
        Self {
            args,
            expression
        }
    }
    pub fn call (&self, call_args: Vec<Statement>) {}
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "function inside jsvalue", )
    }
}