use std::fs::read_to_string;
mod scanner;

fn main() {
    let _buffer = read_to_string("example/example.ts").unwrap();
    scanner::scan(&_buffer)
}
