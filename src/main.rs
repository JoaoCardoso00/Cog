mod frontend;
mod helpers;
mod runtime;

use crate::frontend::parser::lib::Parser;
use crate::helpers::read_file_contents::read_file_contents_from_args;
use runtime::environment::Environment;
use runtime::interpreter::lib::evaluate;

fn main() {
    let (file_contents, should_print_ast) = read_file_contents_from_args();
    let mut parser = Parser::new(file_contents);
    let env = Environment::new(None);

    let ast = parser.parse();

    if should_print_ast {
        println!("{:#?}", ast);
        return;
    }

    evaluate(ast, env);
}
