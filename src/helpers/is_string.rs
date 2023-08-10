pub trait LiteralHelpers {
    fn is_string_literal(&self) -> bool;
}

impl LiteralHelpers for &str {
    fn is_string_literal(&self) -> bool {
        self.ends_with("\"") && self.starts_with("\"")
    }
}
