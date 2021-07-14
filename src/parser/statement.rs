use std::fmt::{Display, Formatter};
use core::fmt;
use crate::lexer::token_kinds::arithmetic_operator::ArithmeticOperator;
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
    Program(Vec<Statement>)
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Statement::FunctionDeclaration(v,s,t) => {
                let mut message = String::new();
                message.push_str(format!("function declaration - name {},", v).as_str());
                message.push_str(" args: ");
                for statement in s {
                    message.push_str(format!("{} ", statement).as_str());
                }
                message.push_str(format!("{} ", t).as_str());
                return writeln!(f,"{} ", message);
            },
            Statement::ConstDeclaration(v,val) => {
                let mut message = String::new();
                message.push_str(format!("const declaration {}", v).as_str());
                message.push_str(" values: \n");
                for statement in val {
                    message.push_str(format!("{} ", statement).as_str());
                }
                return writeln!(f,"{} ", message);

            },
            Statement::Block(v) => {
                writeln!(f, " Block: ");
                for statement in v {
                    writeln!(f,"{}", statement);
                }
                Ok(())
            }
            Statement::TypedArgument(v,s) => write!(f, "typed arguments ({}, {})", v, s),
            Statement::Primitive(v) => write!(f, "value {}", v),
            Statement::VariableRef(v) => write!(f,"variable ref with name {}",v),
            Statement::Call(v,_) => write!(f,"call name {}", v),
            Statement::ArithmeticOperation(v,s,t) => write!(f,"arithmetic operation {} {} {}", v,s,t),
            Statement::Return(v) => write!(f,"return statement: {}", v),
            Statement::Program(v) => {
                write!(f, " Program: ");
                writeln!(f);
                for statement in v {
                    writeln!(f,"statement: {}, ", statement);
                }
                Ok(())
            }
        }
    }
}
