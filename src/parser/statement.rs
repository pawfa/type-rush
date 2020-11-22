use std::fmt::{Display, Formatter};
use core::fmt;
use crate::lexer::tokens::arithmetic_operator::ArithmeticOperator;
use crate::parser::value::PrimitiveValue;

#[derive(Clone, PartialEq)]
pub enum Statement {
    //name, arguments, body
    FunctionDeclaration(String, Vec<Statement>, Box<Statement>),
    ConstDeclaration(String, Vec<Statement>),
    //reference to some variable or function
    VariableRef(String),
    Primitive(PrimitiveValue),
    //name, arguments
    Call(String, Vec<Statement>),
    Return(Box<Statement>),
    Block(Vec<Statement>),
    ArithmeticOperation(ArithmeticOperator,Box<Statement>, Box<Statement>),
    //name, type
    TypedArgument(String, String),
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Statement::FunctionDeclaration(v,s,t) => {
                write!(f, "function declaration - name {},", v);
                write!(f, " args: ");
                for statement in s {
                    write!(f,"{} ", statement);
                }
                write!(f, " block: ");
                write!(f,"{} ", t);
                Ok(())
            },
            Statement::ConstDeclaration(v,_) => write!(f, "const declaration {}", v),
            Statement::Block(v) => {
                write!(f, " Block: ");
                writeln!(f);
                for statement in v {
                    writeln!(f,"in block: {}, ", statement);
                }
                Ok(())
            }
            Statement::TypedArgument(v,s) => write!(f, "typed arguments ({}, {})", v, s),
            Statement::Primitive(v) => write!(f, "value {}", v),
            Statement::VariableRef(v) => write!(f,"variable ref with name {}",v),
            Statement::Call(v,_) => write!(f,"call name {}", v),
            Statement::ArithmeticOperation(v,s,t) => write!(f,"arithmetic operation {} {} {}", v,s,t),
            Statement::Return(v) => write!(f,"return statement: {}", v),
        }
    }
}
