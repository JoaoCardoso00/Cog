#[derive(Debug)]
pub enum Type {
    Keyword,
    Identifier,
    Operator,
    Number,
    String,
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
}
