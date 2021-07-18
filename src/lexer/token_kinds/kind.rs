use crate::lexer::token_kinds::{keyword::Keyword, flow::FlowControl, iteration::Iteration};
use crate::lexer::token_kinds::parenthesis::Parenthesis;
use crate::lexer::token_kinds::double_comparison::DoubleComparison;
use crate::lexer::token_kinds::triple_comparison::TripleComparison;
use std::fmt::{Display, Formatter};
use core::fmt;
use crate::lexer::token_kinds::arithmetic_operator::ArithmeticOperator;
use crate::lexer::token_kinds::literal::Literal;

#[derive(Clone,PartialEq)]
pub enum TokenKind {
    Illegal(char),
    Assignment(char),
    ArithmeticOperator(ArithmeticOperator),
    SingleComparison(char),
    DoubleComparison(DoubleComparison),
    TripleComparison(TripleComparison),
    Parenthesis(Parenthesis),
    Punctuator(char),
    Keyword(Keyword),
    Identifier(String),
    ControlFlow(FlowControl),
    Iteration(Iteration),
    Literal(Literal)
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TokenKind::Assignment(v) => {
                return write!(f, "Kind: Assignment \nValue: {} \n", v);
            },
            TokenKind::ArithmeticOperator(v) => {
                return write!(f, "Kind: ArithmeticOperator \nValue: {} \n", v);
            },
            TokenKind::SingleComparison(v) => {
                return write!(f, "Kind: SingleComparison \nValue: {} \n", v);
            },
            TokenKind::DoubleComparison(v) => {
                return write!(f, "Kind: DoubleComparison \nValue: {} \n", v);
            },
            TokenKind::TripleComparison(v) => {
                return write!(f, "Kind: TripleComparison \nValue: {} \n", v);
            },
            TokenKind::Parenthesis(v) => {
                return write!(f, "Kind: Parenthesis \nValue: {} \n", v);
            },
            TokenKind::Punctuator(v) => {
                return write!(f, "Kind: Punctuator \nValue: {} \n", v);
            },
            TokenKind::Keyword(v) => {
                return write!(f, "Kind: Keyword \nValue: {} \n", v);
            },
            TokenKind::Identifier(v) => {
                return write!(f, "Kind: Identifier \nValue: {} \n", v);
            },
            TokenKind::Literal(v) => {
                return write!(f, "Kind: Literal \nValue: {} \n", v);
            },
            _ => write!(f, "invalid token kind")
        }
    }
}









