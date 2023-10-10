use crate::lexer::lib::Value;

#[derive(Debug)]
pub struct AST<'a> {
    pub(crate) kind: &'a str,
    pub statements: Vec<ASTExpression>,
}

#[derive(Debug)]
pub enum ASTStatementKind {
    ExpressionStatement(ASTExpression),
    VariableDeclaration,
}

#[derive(Debug)]
pub enum ASTExpressionKind {
    Identifier,
    StringLiteral,
    NumericLiteral,
    NullLiteral,
    BinaryExpression,
}

#[derive(Debug)]
pub struct ASTExpression {
    pub(crate) kind: ASTExpressionKind,
    pub(crate) body: ASTExpressionBody,
}

#[derive(Debug)]
pub enum ASTExpressionBody {
    Value(Value),
    BinaryExpressionBody(BinaryExpressionBody),
}

#[derive(Debug)]
pub struct BinaryExpressionBody {
    pub(crate) left: Box<ASTExpression>,
    pub(crate) operator: Value,
    pub(crate) right: Box<ASTExpression>,
}

#[derive(Debug)]
pub struct ASTStatement {
    kind: ASTStatementKind,
}
