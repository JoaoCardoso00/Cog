use std::env;
mod frontend;
mod helpers;
mod runtime;

use crate::{frontend::lexer::lib::tokenize, frontend::parser::lib::Parser};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file_contents =
        std::fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let tokens = tokenize(&file_contents);

    let mut parser = Parser {
        tokens: tokens.unwrap(),
        cursor: 0,
    };

    let ast = parser.parse();

    dbg!(ast);
}
