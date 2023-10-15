use crate::frontend::lexer::lib::Value;

#[derive(Debug)]
pub struct AST<'a> {
    pub(crate) kind: &'a str,
    pub(crate) statements: Vec<ASTStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTStatementKind {
    ExpressionStatement(ASTExpression),
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub(crate) constant: bool,
    pub(crate) identifier: Value,
    pub(crate) value: Option<ASTExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssignment {
    pub(crate) assignee: Box<ASTExpression>,
    pub(crate) value: Box<ASTExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTExpressionKind {
    Identifier,
    AssignmentExpression,
    BinaryExpression,

    // literals
    StringLiteral,
    Property,
    ObjectLiteral,
    NumericLiteral,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTExpression {
    pub(crate) kind: ASTExpressionKind,
    pub(crate) body: ASTExpressionBody,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTExpressionBody {
    Value(Value),
    BinaryExpressionBody(BinaryExpressionBody),
    AssignmentExpressionBody(VariableAssignment),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpressionBody {
    pub(crate) left: Box<ASTExpression>,
    pub(crate) operator: Value,
    pub(crate) right: Box<ASTExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTStatement {
    pub(crate) kind: ASTStatementKind,
}
