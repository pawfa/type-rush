use std::fs::read_to_string;

use engine::runner::Runner;
use lexer::tokenizer::Tokenizer;
use parser::parser::Parser;

mod lexer;
mod parser;
mod engine;

fn main() {
    let buffer = read_to_string("example/example.ts").unwrap();
    let mut tokenizer = Tokenizer::new(&buffer);
    match tokenizer.lex() {
        Ok(result) => {
            for token in &result {
                println!("{}",token)
            }
            let mut parser = Parser::new(&result);
            parser.parse_all();
            println!("{}",parser.statements.len());
            // for statement in parser.statements {
            //     println!("{}",statement)
            // }

            let mut runner = Runner::new();
            runner.start(parser.statements);
        }
        Err(err) => println!("{}", err)
    }



}

