use core::panic;

use crate::{
    frontend::lexer::lib::{Token, Type, Value},
    frontend::{lexer::lib::tokenize, parser::ast::BinaryExpressionBody},
};

use super::ast::{
    ASTExpression, ASTExpressionBody, ASTExpressionKind, ASTStatement, ASTStatementKind, AST,
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub cursor: usize,
}

impl Parser {
    pub fn new(file_contents: String) -> Self {
        let tokens = tokenize(&file_contents).unwrap();
        Self { tokens, cursor: 0 }
    }

    fn not_eof(&self) -> bool {
        self.tokens[self.cursor].r#type != Type::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.cursor].clone()
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.cursor].clone();

        self.cursor += 1;

        token
    }

    pub fn parse(&mut self) -> AST<'static> {
        let mut statements: Vec<ASTStatement> = vec![];

        while self.not_eof() {
            statements.push(self.parse_statement());
        }

        AST {
            kind: "Program",
            statements,
        }
    }

    fn parse_statement(&mut self) -> ASTStatement {
        let expression = self.parse_expression();

        ASTStatement {
            kind: ASTStatementKind::ExpressionStatement(expression),
        }
    }

    fn parse_expression(&mut self) -> ASTExpression {
        self.parse_additive_expression()
    }

    fn parse_additive_expression(&mut self) -> ASTExpression {
        let mut left = self.parse_multiplicative_expression();

        while self.peek().value == Value::String("+".to_string())
            || self.peek().value == Value::String("-".to_string())
        {
            let operator = self.advance().value;

            let right = self.parse_multiplicative_expression();

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

    fn parse_multiplicative_expression(&mut self) -> ASTExpression {
        let mut left = self.parse_primary_expression();

        while self.peek().value == Value::String("*".to_string())
            || self.peek().value == Value::String("/".to_string())
            || self.peek().value == Value::String("%".to_string())
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
        let token = self.advance();

        match token.r#type {
            Type::Identifier => ASTExpression {
                kind: ASTExpressionKind::Identifier,
                body: ASTExpressionBody::Value(token.value),
            },
            Type::OpenParen => {
                let value = self.parse_expression();
                let closing_paren = self.advance();

                if closing_paren.r#type != Type::CloseParen {
                    panic!("expected closing parenthesis");
                }

                value
            }
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
