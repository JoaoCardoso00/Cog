use core::panic;

use crate::{
    lexer::lib::{Token, Type, Value},
    parser::ast::BinaryExpressionBody,
};

use super::ast::{ASTExpression, ASTExpressionBody, ASTExpressionKind, ASTStatement, AST};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn parse(&mut self) -> AST<'static> {
        let mut statements: Vec<ASTExpression> = vec![];

        while self.peek().r#type != Type::EOF {
            statements.push(self.parse_statement());
        }

        AST {
            kind: "Program",
            statements,
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.cursor].clone()
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.cursor].clone();

        self.cursor += 1;

        token
    }

    fn parse_statement(&mut self) -> ASTExpression {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> ASTExpression {
        self.parse_additive_expression()
    }

    //TODO: fix this
    fn parse_additive_expression(&mut self) -> ASTExpression {
        let mut left = self.parse_primary_expression();

        self.cursor += 1;

        while self.peek().value == Value::String("+".to_string())
            || self.peek().value == Value::String("-".to_string())
        {
            let operator = self.advance().value;

            let right = self.parse_primary_expression();

            left = ASTExpression {
                kind: ASTExpressionKind::BinaryExpression,
                body: ASTExpressionBody::BinaryExpressionBody(BinaryExpressionBody {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }),
            };
        }

        left
    }

    fn parse_primary_expression(&mut self) -> ASTExpression {
        let token = self.peek();

        match token.r#type {
            Type::Identifier => ASTExpression {
                kind: ASTExpressionKind::Identifier,
                body: ASTExpressionBody::Value(token.value),
            },
            Type::Number => ASTExpression {
                kind: ASTExpressionKind::NumericLiteral,
                body: ASTExpressionBody::Value(token.value),
            },
            Type::String => ASTExpression {
                kind: ASTExpressionKind::StringLiteral,
                body: ASTExpressionBody::Value(token.value),
            },
            _ => panic!("unexpected token found during parsing: {:?}", token),
        }
    }
}
