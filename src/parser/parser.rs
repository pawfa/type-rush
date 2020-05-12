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
use std::borrow::Borrow;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    statements: Vec<Statement>,
    pos: usize
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            statements: Vec::new(),
            pos: 0
        }
    }

    fn get_token(&self, pos: usize) -> Result<Token, ParserError> {
        if pos < self.tokens.len() {
            Ok(self.tokens.get(pos).expect("Error getting token").clone())
        } else {
            Err(ParserError::GetToken)
        }
    }

    fn increment(&mut self) {
        self.pos = self.pos +1
    }

    pub fn parse_all(&mut self) {
        while self.pos < self.tokens.len() {
            let result = self.parse();
            match result {
                Ok(result ) => self.statements.push(result),
                Err(result) => println!("{} err", result)
            }
        }
    }

    pub fn parse(&mut self) -> Result<Statement, ParserError> {
            let token = self.get_token(self.pos)?.kind;

            self.increment();
            return match token {
                TokenKind::Keyword(Keyword::Function) => {
                    loop {
                        let func_token = self.get_token(self.pos)?.kind;
                        self.increment();

                        let name = match func_token {
                            TokenKind::Identifier(ref name) => name.clone(),
                            _ => {
                                println!("{} err", func_token);
                                return Err(ParserError::FunctionName)
                            }
                        };
                        let args = self.parse_function_parameters();
                        println!("{} arg", args.unwrap().get(1).unwrap());

                        return Ok(Statement::FunctionDeclaration(name,Vec::new(),Box::new(Statement::Block)))
                    }
                },
                _ => Err(ParserError::Generic)
        };
    }

    pub fn parse_function_parameters(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut params = Vec::new();

        loop {
            let token = self.get_token(self.pos)?.kind;
            self.increment();

            if token == TokenKind::Parenthesis(Parenthesis::RParen) {
                return Ok(params)
            }

            match token {
                TokenKind::Identifier(token) => {
                    let arg_name = token.clone();

                    loop {
                        let arg_token = self.get_token(self.pos)?.kind;
                        self.increment();
                        match arg_token {
                            TokenKind::Identifier(arg_token) => {
                                params.push(Statement::TypedArgument(arg_name, arg_token));
                                break
                            },
                            TokenKind::Punctuator(':') => {},
                            TokenKind::Punctuator(',') => {},
                            _ => {}
                        }

                    }


                },
                _ => {}
            }
        }

    }
}