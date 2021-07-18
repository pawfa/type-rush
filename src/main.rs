use std::fs::read_to_string;

use engine::runner::Runner;
use lexer::lexer::Lexer;
use parser::parser::Parser;
use crate::type_checker::type_checker::TypeChecker;
use crate::parser::statement::Statement;

mod lexer;
mod parser;
mod engine;
mod type_checker;

fn main() {
    let analysed_code = read_to_string("example/example.ts").unwrap();

    let tokens = Lexer::new(&analysed_code).analyse().unwrap();
    // for token in &tokens {
    //     println!("{}", token);
    // }
    let statement = Parser::new(tokens).parse_all();

    match TypeChecker::new(statement).check() {
        Ok(checked_tree) => Runner::new().start(checked_tree),
        Err(err) => println!("{}",err)
    };



}
