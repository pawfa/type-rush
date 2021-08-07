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
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut records = String::new();
        for record in &self.args {
            records.push_str( "[");
            records.push_str( "statement: ");
            records.push_str( record.to_string().as_str());
            records.push_str( "]");
        }
        return write!(f, "Function: \n expression:\n {} \n records: {}",self.expression, records );
    }
}