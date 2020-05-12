use std::fmt::{Display, Formatter};
use core::fmt;

pub enum Statement {
    FunctionDeclaration(String, Vec<Statement>, Box<Statement>),
    ExpressionStatement,
    Block,
    TypedArgument(String, String),
    Interface
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Statement::FunctionDeclaration(v,s,t) => v.fmt(f),
            Statement::ExpressionStatement => write!(f,"expression statement"),
            Statement::Block => write!(f,"block"),
            Statement::TypedArgument(v,s) => write!(f, "({}, {})", v, s),
            Statement::Interface => write!(f,"interface"),
            _ => write!(f, "invalid statement kind")
        }
    }
}
