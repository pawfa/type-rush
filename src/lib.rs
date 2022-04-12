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

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn run(analysed_code: &str) {
    let tokens = Lexer::new(&analysed_code.to_string()).analyse().unwrap();

    let statement = Parser::new(tokens).parse_all();

    match TypeChecker::new(statement).check() {
        Ok(checked_tree) => Runner::new().start(checked_tree),
        Err(err) => log(err.to_string().as_str())
    };
}
