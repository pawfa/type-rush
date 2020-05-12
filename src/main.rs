use std::fs::read_to_string;
use parser::parser::Parser;

mod lexer;
mod parser;

fn main() {
    let buffer = read_to_string("example/example.ts").unwrap();
    let mut tokenizer = lexer::tokenizer::Tokenizer::new(&buffer);
    tokenizer.lex();
    let vector = tokenizer.tokens;
    let mut parser = Parser::new(&vector);
    parser.parse_all();
}