use std::fs::read_to_string;

use engine::runner::Runner;
use lexer::lexer::Lexer;
use parser::parser::Parser;

mod lexer;
mod parser;
mod engine;

fn main() {
    let analysed_code = read_to_string("example/example.ts").unwrap();

    let tokens = Lexer::new(&analysed_code).analyse().unwrap();

    let statement = Parser::new(tokens).parse_all();

    Runner::new().start(statement);
}
