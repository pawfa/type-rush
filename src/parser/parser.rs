use crate::lexer::token::Token;
use crate::lexer::tokens::keyword::Keyword;
use crate::parser::parse_errors::ParserError;
use crate::parser::statement::Statement;
use crate::lexer::tokens::kind::TokenKind;
use crate::lexer::tokens::parenthesis::Parenthesis;
use crate::parser::statement::Statement::ExpressionStatement;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    statements: Vec<Statement>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            statements: Vec::new(),
            pos: 0,
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
        self.pos = self.pos + 1
    }

    pub fn parse_all(&mut self) {
        while self.pos < self.tokens.len() {
            let result = self.parse();
            match result {
                Ok(result) => self.statements.push(result),
                Err(result) => {
                    // println!("{:?}", result)

                }
            }
        }
    }

    pub fn parse(&mut self) -> Result<Statement, ParserError> {
        let token = self.get_token(self.pos)?.kind;

        self.increment();
        return match token {
            TokenKind::Parenthesis(Parenthesis::LBrace) => {
                let mut block_expressions = Vec::new();
                loop {

                    if self.get_token(self.pos)?.kind == TokenKind::Parenthesis(Parenthesis::RBrace) {
                        break;
                    } else {
                        block_expressions.push(self.parse()?)
                    }
                }
                self.increment();
                println!("{} length", block_expressions.len());
                return Ok(Statement::Block(block_expressions));
            }
            TokenKind::Keyword(Keyword::Const) => {
                let const_name = match self.get_token(self.pos) {
                    Ok(Token {
                           kind: TokenKind::Identifier(const_name),
                           ..
                       }) => {
                        const_name.clone()
                    }
                    Ok(Token) => return Err(ParserError::Message("Const name is required")),
                    Err(_) => {
                        return Err(ParserError::GetToken);
                    }
                };
                self.increment();
                let const_value = match self.get_token(self.pos) {
                    Ok(Token {
                           kind: TokenKind::Assignment('='),
                           ..
                       }) => {
                        self.increment();
                        self.parse()?
                    },
                    Ok(Token) => return Err(ParserError::Message("Assignment error")),
                    Err(_) => {
                        return Err(ParserError::GetToken);
                    }
                };

                println!("{} const name", const_value);

                return Ok(Statement::ConstDeclaration(const_name, Box::new(const_value)));
            }
            TokenKind::Keyword(Keyword::Function) => {
                loop {
                    let func_token = self.get_token(self.pos)?.kind;
                    self.increment();

                    let name = match func_token {
                        TokenKind::Identifier(ref name) => name.clone(),
                        _ => {
                            println!("{} err", func_token);
                            return Err(ParserError::FunctionName);
                        }
                    };
                    let args = self.parse_function_parameters().unwrap();

                    //consumed left bracket for function
                    let func_body = self.get_token(self.pos)?.kind;
                    if func_body != TokenKind::Parenthesis(Parenthesis::LBrace) {
                        return Err(ParserError::Message("Expected bracket"));
                    }

                    let body = self.parse()?;

                    return Ok(Statement::FunctionDeclaration(name, args, Box::new(body)));
                }
            }
            y => {
                println!("{} not matched", y);
                Err(ParserError::Generic)
            }
        };
    }

    pub fn parse_function_parameters(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut params = Vec::new();

        loop {
            let token = self.get_token(self.pos)?.kind;
            self.increment();

            if token == TokenKind::Parenthesis(Parenthesis::RParen) {
                return Ok(params);
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
                                break;
                            }
                            TokenKind::Punctuator(':') => {}
                            TokenKind::Punctuator(',') => {}
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}