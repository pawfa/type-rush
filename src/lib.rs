use engine::runner::Runner;
use lexer::lexer::Lexer;
use parser::parser::Parser;
use crate::type_checker::type_checker::TypeChecker;

mod lexer;
mod parser;
mod engine;
mod type_checker;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run(analysed_code: &str) {
    log("Engine starting");
    let tokens = Lexer::new(&analysed_code.to_string()).analyse().unwrap();

    let statement = Parser::new(tokens).parse_all();

    match TypeChecker::new(statement).check() {
        Ok(checked_tree) => Runner::new().start(checked_tree),
        Err(err) => log(err.to_string().as_str())
    };
}
