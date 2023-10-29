use std::env;

pub fn read_file_contents_from_args() -> (String, bool, bool) {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut should_print_ast = false;
    let mut should_print_tokens = false;

    if args.contains(&"-ast".to_string()) {
        should_print_ast = true;
    }

    if args.contains(&"-tokens".to_string()) {
        should_print_tokens = true;
    }

    let file_contents =
        std::fs::read_to_string(file_path).expect("Something went wrong reading the file");

    (file_contents, should_print_ast, should_print_tokens)
}
