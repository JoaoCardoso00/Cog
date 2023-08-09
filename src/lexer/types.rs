#[derive(Debug)]
pub enum Type {
    Keyword,
    Variable,
    Expression,
    Number,
    String,
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
}
