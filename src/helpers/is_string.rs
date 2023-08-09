pub trait literal_helpers {
    fn is_string_literal(&self) -> bool;
}

impl literal_helpers for &str {
    fn is_string_literal(&self) -> bool {
        self.ends_with("\"") && self.starts_with("\"")
    }
}
