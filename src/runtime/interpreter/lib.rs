use crate::{
    frontend::parser::ast::AST,
    runtime::{environment::Environment, eval::statements::evaluate_program, values::RuntimeValue},
};

pub fn evaluate(ast: AST, env: Environment) -> RuntimeValue {
    match ast.kind {
        "Program" => evaluate_program(ast, env),
        _ => panic!("Unknown AST kind: {}", ast.kind),
    }
}
