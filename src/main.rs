use std::fs::read_to_string;
use parser::parser::Parser;
use engine::runner::Runner;

mod lexer;
mod parser;
mod engine;

fn main() {
    let buffer = read_to_string("example/example.ts").unwrap();
    let mut tokenizer = lexer::tokenizer::Tokenizer::new(&buffer);
    tokenizer.lex();
    let vector = tokenizer.tokens;
    let mut parser = Parser::new(&vector);
    parser.parse_all();

    let mut runner = Runner::new();
    runner.start(parser.statements);
}