mod frontend;
mod helpers;
mod runtime;

use crate::frontend::parser::lib::Parser;
use crate::helpers::build_bool_runtime_value::build_bool_runtime_value;
use crate::helpers::build_null_runtime_value::build_null_runtime_value;
use crate::helpers::build_number_runtime_value::build_number_runtime_value;
use crate::helpers::read_file_contents::read_file_contents_from_args;
use runtime::environment::Environment;
use runtime::interpreter::lib::evaluate;

fn main() {
    let file_contents = read_file_contents_from_args();
    let mut parser = Parser::new(file_contents);
    let mut env = Environment::new(None);

    // Global variables
    env.declare_variable("x".to_string(), build_number_runtime_value(10.0));
    env.declare_variable("true".to_string(), build_bool_runtime_value(true));
    env.declare_variable("false".to_string(), build_bool_runtime_value(false));
    env.declare_variable("null".to_string(), build_null_runtime_value());

    let ast = parser.parse();

    let res = evaluate(ast, env);

    dbg!(res);
}
