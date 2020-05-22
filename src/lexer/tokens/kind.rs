use crate::lexer::tokens::{keyword::Keyword, flow::FlowControl, iteration::Iteration};
use crate::lexer::tokens::parenthesis::Parenthesis;
use crate::lexer::tokens::double_comparison::DoubleComparison;
use crate::lexer::tokens::triple_comparison::TripleComparison;
use std::fmt::{Display, Formatter};
use core::fmt;
use crate::lexer::tokens::arithmetic_operator::ArithmeticOperator;
use crate::lexer::tokens::literal::Literal;

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
            TokenKind::Assignment(v) => v.fmt(f),
            TokenKind::ArithmeticOperator(v) => v.fmt(f),
            TokenKind::SingleComparison(v) => v.fmt(f),
            TokenKind::DoubleComparison(v) => v.fmt(f),
            TokenKind::TripleComparison(v) => v.fmt(f),
            TokenKind::Parenthesis(v) => v.fmt(f),
            TokenKind::Punctuator(v) => v.fmt(f),
            TokenKind::Keyword(v) => v.fmt(f),
            TokenKind::Identifier(v) => v.fmt(f),
            _ => write!(f, "invalid token kind")
        }
    }
}









