use std::fs::read_to_string;

mod lexer;

fn main() {
    let buffer = read_to_string("example/example.ts").unwrap();
    let mut tokenizer = lexer::tokenizer::Tokenizer::new(&buffer);
    tokenizer.lex();
    let vector = tokenizer.tokens;
    for x in &vector {
        println!("{}", x);
    }
}