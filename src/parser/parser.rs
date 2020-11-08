use crate::lexer::token::Token;
use crate::lexer::tokens::keyword::Keyword;
use crate::parser::parse_errors::ParserError;
use crate::parser::statement::Statement;
use crate::lexer::tokens::kind::TokenKind;
use crate::lexer::tokens::parenthesis::Parenthesis;
use crate::lexer::tokens::arithmetic_operator::ArithmeticOperator;
use crate::parser::value::PrimitiveValue;
use crate::lexer::tokens::literal::Literal;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    pub statements: Vec<Statement>,
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
                Err(_) => {}
            }
        }
    }

    fn parse(&mut self) -> Result<Statement, ParserError> {
        let token = self.get_token(self.pos)?.kind;

        self.increment();
        return match token {
            TokenKind::Keyword(Keyword::Return) => {
                Ok(Statement::Return(Box::new(self.parse()?)))
            },
            TokenKind::Identifier(token) => {
                let next_token = self.get_token(self.pos)?.kind;
                if next_token == TokenKind::Parenthesis(Parenthesis::LParen) {
                    self.increment();
                    let function_call_args = self.parse_call_parameters()?;
                    return Ok(Statement::Call(token, function_call_args));
                }
                return Ok(Statement::VariableRef(token));
            }
            TokenKind::ArithmeticOperator(token) => {
                let operation = self.parse_arithmetic_operation(token);
                return operation;
            }
            TokenKind::Parenthesis(Parenthesis::LBrace) => {
                let mut block_expressions = Vec::new();
                loop {
                    if self.get_token(self.pos)?.kind == TokenKind::Parenthesis(Parenthesis::RBrace) {
                        break;
                    }
                    block_expressions.push(self.parse()?)
                }
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
                    Ok(_) => return Err(ParserError::Message("Const name is required")),
                    Err(_) => {
                        return Err(ParserError::GetToken);
                    }
                };

                self.increment();
                let const_value = match self.get_token(self.pos) {
                    Ok(Token {
                           kind: TokenKind::Assignment('='),
                           ..
                       }) =>
                        match self.parse() {
                            Ok(v) => v,
                            Err(t) => {
                                return Err(ParserError::Message("Assignment error"))
                            }
                        },
                    _ => return Err(ParserError::Message("Assignment error")),
                };

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

                    //checked if left bracket is present
                    let func_body = self.get_token(self.pos)?.kind;
                    if func_body != TokenKind::Parenthesis(Parenthesis::LBrace) {
                        return Err(ParserError::Message("Expected bracket"));
                    }

                    return Ok(Statement::FunctionDeclaration(name, args, Box::new(self.parse()?)));
                }
            }
            _ => {
                return self.parse();
            }
        };
    }

    fn parse_arithmetic_operation(&mut self, operator: ArithmeticOperator) -> Result<Statement, ParserError> {
        let first_variable =
            match self.get_token(self.pos - 2)?.kind {
                TokenKind::Literal(first_variable) => match first_variable {
                    Literal::Boolean(v) => Statement::Primitive(PrimitiveValue::Boolean(v)),
                    Literal::String(v) => Statement::Primitive(PrimitiveValue::String(v)),
                    Literal::Num(v) => Statement::Primitive(PrimitiveValue::Num(v))
                },
                TokenKind::Identifier(first_variable) => Statement::VariableRef(first_variable),
                _ => return Err(ParserError::Message("Should be identifier"))
            };

        let second_variable =
            match self.get_token(self.pos)?.kind {
                TokenKind::Literal(second_variable) => match second_variable {
                    Literal::Boolean(v) => Statement::Primitive(PrimitiveValue::Boolean(v)),
                    Literal::String(v) => Statement::Primitive(PrimitiveValue::String(v)),
                    Literal::Num(v) => Statement::Primitive(PrimitiveValue::Num(v))
                },
                TokenKind::Identifier(second_variable) => Statement::VariableRef(second_variable),
                _ => return Err(ParserError::Message("Should be identifier"))
            };
        self.increment();

        return Ok(Statement::ArithmeticOperation(operator, Box::new(first_variable), Box::new(second_variable)));
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Statement>, ParserError> {
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

    fn parse_call_parameters(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut params = Vec::new();

        loop {
            let token = self.get_token(self.pos)?.kind;
            self.increment();

            if token == TokenKind::Parenthesis(Parenthesis::RParen) {
                return Ok(params);
            }

            match token {
                TokenKind::Literal(Literal::Num(token)) => params.push(Statement::Primitive(PrimitiveValue::Num(token))),
                _ => {}
            }
        }
    }
}