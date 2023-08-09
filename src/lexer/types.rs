#[derive(Debug)]
pub enum Type {
    Keyword,
    Variable,
    Expression,
    Number,
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
}
