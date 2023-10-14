mod frontend;
mod helpers;
mod runtime;

use crate::frontend::parser::lib::Parser;
use crate::helpers::build_bool_runtime_value::build_bool_runtime_value;
use crate::helpers::build_null_runtime_value::build_null_runtime_value;
use crate::helpers::read_file_contents::read_file_contents_from_args;
use runtime::environment::Environment;
use runtime::interpreter::lib::evaluate;

fn main() {
    let file_contents = read_file_contents_from_args();
    let mut parser = Parser::new(file_contents);
    let mut env = Environment::new(None);

    // Global variables
    env.declare_variable("true".to_string(), build_bool_runtime_value(true), true);
    env.declare_variable("false".to_string(), build_bool_runtime_value(false), true);
    env.declare_variable("null".to_string(), build_null_runtime_value(), true);

    let ast = parser.parse();
    let res = evaluate(ast, env);

    dbg!(res);
}
