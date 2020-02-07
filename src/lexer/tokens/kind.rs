use crate::lexer::tokens::{keyword::Keyword, literal::Literal, flow::FlowControl, iteration::Iteration};
use crate::lexer::tokens::punctuator::Punctuator;
use crate::lexer::tokens::parenthesis::Parenthesis;
use crate::lexer::tokens::double_comparison::DoubleComparison;
use crate::lexer::tokens::assignment::Assignment;
use crate::lexer::tokens::triple_comparison::TripleComparison;
use std::fmt::{Display, Formatter, Error};
use core::fmt;

pub enum TokenKind {
//    Eof,
    Illegal(char),

//    Ident(String),
//    Int(String),
//    Asterisk,
//    Minus,
//    Plus,

    Assignment(Assignment),
    SingleComparison(char),
    DoubleComparison(DoubleComparison),
    TripleComparison(TripleComparison),
    Parenthesis(Parenthesis),
    Punctuator(char),
    Keyword(Keyword),
//    Literal(Literal),
    Identifier(String),
    ControlFlow(FlowControl),
    Iteration(Iteration),
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TokenKind::Assignment(v) => v.fmt(f),
            TokenKind::SingleComparison(v) => v.fmt(f),
            TokenKind::DoubleComparison(v) => v.fmt(f),
            TokenKind::TripleComparison(v) => v.fmt(f),
            TokenKind::Parenthesis(v) => v.fmt(f),
            TokenKind::Punctuator(v) => v.fmt(f),
            TokenKind::Keyword(v) => v.fmt(f),
//            TokenKind::Literal(v) => v.fmt(f),
            TokenKind::Identifier(v) => v.fmt(f),
//            TokenKind::ControlFlow(v) => v.fmt(f),
//            TokenKind::Iteration(v) => v.fmt(f),
            _ => write!(f, "invalid token")
        }
    }
}









