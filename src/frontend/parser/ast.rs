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
    FunctionDeclaration(FunctionDeclaration),
    ConditionalStatement(ConditionalStatement),
    LoopStatement(LoopStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopStatement {
    pub(crate) body: Vec<ASTStatement>,
    pub(crate) identifier: String,
    pub(crate) start: f64,
    pub(crate) end: f64,
    pub(crate) condition: Option<ASTExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalStatement {
    pub(crate) condition: Option<ASTExpression>,
    pub(crate) consequence: Vec<ASTStatement>,
    pub(crate) alternate: Option<Box<ASTStatement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub(crate) constant: bool,
    pub(crate) identifier: Value,
    pub(crate) value: Option<ASTExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub(crate) identifier: String,
    pub(crate) parameters: Vec<String>,
    pub(crate) body: Vec<ASTStatement>,
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
    MemberExpression,
    CallExpression,
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
    BinaryExpressionBody(BinaryExpression),
    AssignmentExpressionBody(VariableAssignment),
    CallExpressionBody(CallExpression),
    MemberExpressionBody(MemberExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub(crate) left: Box<ASTExpression>,
    pub(crate) operator: Value,
    pub(crate) right: Box<ASTExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression {
    pub(crate) arguments: Vec<ASTExpression>,
    pub(crate) caller: Box<ASTExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpression {
    pub(crate) object: Box<ASTExpression>,
    pub(crate) property: Box<ASTExpression>,
    pub(crate) computed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTStatement {
    pub(crate) kind: ASTStatementKind,
}
