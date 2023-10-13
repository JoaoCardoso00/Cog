use std::env;
mod frontend;
mod helpers;
mod runtime;

use crate::helpers::build_bool_runtime_value::build_bool_runtime_value;
use crate::helpers::build_null_runtime_value::build_null_runtime_value;
use crate::helpers::build_number_runtime_value::build_number_runtime_value;
use crate::{frontend::lexer::lib::tokenize, frontend::parser::lib::Parser};
use runtime::environment::Environment;
use runtime::interpreter::lib::evaluate;

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

    let mut env = Environment::new(None);

    env.declare_variable("x".to_string(), build_number_runtime_value(10.0));
    env.declare_variable("true".to_string(), build_bool_runtime_value(true));
    env.declare_variable("false".to_string(), build_bool_runtime_value(false));
    env.declare_variable("null".to_string(), build_null_runtime_value());

    let ast = parser.parse();

    let res = evaluate(ast, env);

    dbg!(res);
}
