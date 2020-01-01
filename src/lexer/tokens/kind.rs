use crate::lexer::tokens::{keyword::Keyword, literal::Literal, flow::FlowControl, iteration::Iteration};

pub enum TokenKind {
    Eof,
    Illegal(char),

    Ident(String),
    Int(String),

    Assign,
    Asterisk,
    Bang,
    Minus,
    Plus,
    Slash,

    Gt,
    Lt,
    Eq,
    NotEq,

    Comma,
    Semicolon,
    Colon,
    QuestionMark,

    LBrace,
    LParen,
    RBrace,
    RParen,
    Keyword(Keyword),
    Literal(Literal),
    Identifier(String),
    ControlFlow(FlowControl),
    Iteration(Iteration),
}









