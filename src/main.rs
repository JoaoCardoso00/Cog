use std::env;

use crate::lexer::index::tokenize;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file_contents =
        std::fs::read_to_string(file_path).expect("Something went wrong reading the file");

    // println!("File contents: {}", file_contents)

    let tokens = tokenize(file_contents);
}
