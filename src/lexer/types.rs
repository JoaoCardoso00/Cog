#[derive(Debug)]
pub enum Type {
    Keyword,
    Variable,
    Operator,
    Number,
    String,
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
}
