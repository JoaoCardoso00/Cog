pub trait LiteralHelpers {
    fn is_string_literal(&self) -> bool;
}

impl LiteralHelpers for &str {
    fn is_string_literal(&self) -> bool {
        println!("{:?}", self.starts_with("\""));
        println!("{:?}", self.chars().collect::<Vec<char>>());

        self.is_ascii() && self.ends_with('"') && self.starts_with("\"")
    }
}

impl LiteralHelpers for String {
    fn is_string_literal(&self) -> bool {
        self.is_ascii() && self.ends_with('"') && self.starts_with("\"")
    }
}
