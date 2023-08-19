use crate::lexer::lib::Value;

#[derive(Debug)]
pub enum Expression {
    Assignment(Box<AssignmentExpression>),
}

#[derive(Debug)]
pub struct AssignmentExpression {
    pub identifier: String,
    pub value: Value
}
