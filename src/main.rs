use engine::runner::Runner;
use lexer::lexer::Lexer;
use parser::parser::Parser;
use std::fs::read_to_string;
use std::io;

mod lexer;
mod parser;
mod engine;

fn main() {
    let analysed_code = read_to_string("example/example.ts").unwrap();

    let mut tokens = Lexer::new(&analysed_code).analyse().unwrap();
    for token in &tokens {
        println!("{}", token)
    }
    let mut parser = Parser::new(&tokens);
    parser.parse_all();
    println!("{}", parser.statements.len());
    // for statement in parser.statements {
    //     println!("{}",statement)
    // }

    let mut runner = Runner::new();
    runner.start(parser.statements);
}


