use crate::frontend::lexer::lib::Value;

#[derive(Debug)]
pub struct AST<'a> {
    pub(crate) kind: &'a str,
    pub(crate) statements: Vec<ASTStatement>,
}

#[derive(Debug)]
pub enum ASTStatementKind {
    ExpressionStatement(ASTExpression),
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub(crate) constant: bool,
    pub(crate) identifier: Value,
    pub(crate) value: Option<ASTExpression>,
}

#[derive(Debug)]
pub enum ASTExpressionKind {
    Identifier,
    StringLiteral,
    NumericLiteral,
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
    pub(crate) kind: ASTStatementKind,
}
