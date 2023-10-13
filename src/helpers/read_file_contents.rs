use std::env;

pub fn read_file_contents_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file_contents =
        std::fs::read_to_string(file_path).expect("Something went wrong reading the file");

    file_contents
}
