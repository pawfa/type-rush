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
            TokenKind::Assignment(v) => v.fmt(f),
            TokenKind::ArithmeticOperator(v) => v.fmt(f),
            TokenKind::SingleComparison(v) => v.fmt(f),
            TokenKind::DoubleComparison(v) => v.fmt(f),
            TokenKind::TripleComparison(v) => v.fmt(f),
            TokenKind::Parenthesis(v) => v.fmt(f),
            TokenKind::Punctuator(v) => v.fmt(f),
            TokenKind::Keyword(v) => v.fmt(f),
            TokenKind::Identifier(v) => v.fmt(f),
            TokenKind::Literal(v) => v.fmt(f),
            _ => write!(f, "invalid token kind")
        }
    }
}









