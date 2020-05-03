use crate::lexer::token::Token;
use crate::lexer::tokens::keyword::Keyword;
use std::fmt::Error;
use core::fmt;
use std::error;
use crate::parser::parse_errors::ParserError;
use crate::parser::statement::Statement;
use std::cell::Cell;
use crate::lexer::tokens::kind::TokenKind;
use crate::lexer::tokens::parenthesis::Parenthesis;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    statements: Vec<Statement>,
    pos: Cell<usize>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            statements: Vec::new(),
            pos: Cell::from(0)
        }
    }

    fn get_token(&self, pos: usize) -> Result<Token, ParserError> {
        if pos < self.tokens.len() {
            Ok(self.tokens.get(pos).expect("Error getting token").clone())
        } else {
            Err(ParserError::GetToken)
        }
    }

    fn increment(&self) {
        self.pos.set(self.pos.get()+1);
    }

    pub fn parse_all(&self) {
        while self.pos.get() < self.tokens.len() {
            let result = self.parse();
        }
    }

    pub fn parse(&self) -> Result<(), ParserError> {
        loop {
            if self.pos.get() > self.tokens.len() {
                break
            }

            let token = self.get_token(self.pos.get())?.kind;

            self.increment();
            match token {
                TokenKind::Keyword(Keyword::Function) => {

                    loop {
                        let func_token = self.get_token(self.pos.get())?.kind;

                        self.increment();

                        if func_token == TokenKind::Parenthesis(Parenthesis::RParen) {
                            break
                        }
                    }
                    println!("{}", token)
                },
                _ => println!("{}", "empty")
            }

        }
        return Ok({})

    }
}