use crate::lexer::token::Token;
use crate::lexer::token_kinds::arithmetic_operator::ArithmeticOperator;
use crate::lexer::token_kinds::keyword::Keyword;
use crate::lexer::token_kinds::kind::TokenKind;
use crate::lexer::token_kinds::literal::Literal;
use crate::lexer::token_kinds::parenthesis::Parenthesis;
use crate::parser::parse_errors::ParserError;
use crate::parser::statement::Statement;
use crate::parser::value::PrimitiveValue;

pub struct Parser {
    tokens: Vec<Token>,
    statement: Statement,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            statement: Statement::Program(Vec::new()),
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

    pub fn parse_all(&mut self) -> Statement {
        while self.pos < self.tokens.len() {
            let result = self.parse();

            match result {
                Ok(result) => {
                    if let Statement::Program(mut content) = self.statement.clone() {
                        content.push(result);
                        self.statement = Statement::Program(content)
                    }
                },
                Err(_) => {}
            }
        }
        return self.statement.clone()
    }

    fn parse(&mut self) -> Result<Statement, ParserError> {
        let token = self.get_token(self.pos)?.kind;

        self.increment();
        return match token {
            TokenKind::Keyword(Keyword::Return) => {
                Ok(Statement::Return(Box::new(self.parse()?)))
            }
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
                    if self.get_token(self.pos + 2)?.kind == TokenKind::Parenthesis(Parenthesis::RBrace) {
                        break;
                    }

                    let token_in_block = self.parse();

                    match token_in_block {
                        Ok(token) => block_expressions.push(token),
                        Err(e) => return Err(e)
                    }
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

                let assignment = self.get_token(self.pos)?;
                if assignment.kind == TokenKind::Assignment('=') {
                    self.increment();
                } else {
                    return Err(ParserError::Message("Assignment error"));
                }

                let mut arguments = Vec::new();

                loop {
                    let token = self.get_token(self.pos)?.kind;

                    if token == TokenKind::Punctuator('\n') || token == TokenKind::Punctuator(';') {
                        return Ok(Statement::ConstDeclaration(const_name, arguments));
                    } else {
                        arguments.push(self.parse()?);
                    }
                }
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
                    let left_bracket = self.get_token(self.pos)?.kind;
                    if left_bracket != TokenKind::Parenthesis(Parenthesis::LBrace) {
                        return Err(ParserError::Message("Expected bracket"));
                    }

                    let func_body = self.parse()?;

                    return Ok(Statement::FunctionDeclaration(name, args, Box::new(func_body)));
                }
            }
            TokenKind::Punctuator('\n') => self.parse(),
            TokenKind::Parenthesis(Parenthesis::RBrace) => self.parse(),
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
                                params.push(Statement::TypedParameter(arg_name, arg_token));
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