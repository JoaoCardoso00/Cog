use core::panic;

use crate::{
    frontend::lexer::lib::{Token, Type, Value},
    frontend::{
        lexer::lib::{tokenize, Object, Property},
        parser::ast::{BinaryExpressionBody, VariableAssignment, VariableDeclaration},
    },
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
        match self.peek().r#type {
            Type::Let | Type::Const => self.parse_variable_declaration(),
            _ => ASTStatement {
                kind: ASTStatementKind::ExpressionStatement(self.parse_expression()),
            },
        }
    }

    fn parse_variable_declaration(&mut self) -> ASTStatement {
        let is_constant = self.advance().r#type == Type::Const;

        let identifier = self.advance();
        let identifier = match identifier.r#type {
            Type::Identifier => identifier.value,
            _ => panic!("expected identifier"),
        };

        if self.peek().r#type == Type::Semi {
            self.advance();
            match is_constant {
                true => panic!("Constants need to be declared with a value, no value provided"),
                false => {
                    return ASTStatement {
                        kind: ASTStatementKind::VariableDeclaration(VariableDeclaration {
                            constant: false,
                            identifier,
                            value: None,
                        }),
                    }
                }
            }
        }

        match self.advance().r#type {
            Type::Equals => (),
            _ => panic!("expected \"=\" at variable declaration"),
        };

        let declaration = ASTStatement {
            kind: ASTStatementKind::VariableDeclaration(VariableDeclaration {
                constant: is_constant,
                identifier,
                value: Some(self.parse_expression()),
            }),
        };

        match self.advance().r#type {
            Type::Semi => (),
            _ => panic!("expected \";\" at variable declaration"),
        };

        declaration
    }

    fn parse_expression(&mut self) -> ASTExpression {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> ASTExpression {
        let left = self.parse_object_expression(); //TODO: switch this out with object_expression

        if self.peek().r#type == Type::Equals {
            self.advance();

            let value = self.parse_assignment_expression();

            return ASTExpression {
                kind: ASTExpressionKind::AssignmentExpression,
                body: ASTExpressionBody::AssignmentExpressionBody(VariableAssignment {
                    assignee: Box::new(left),
                    value: Box::new(value),
                }),
            };
        }

        left
    }

    fn parse_object_expression(&mut self) -> ASTExpression {
        if self.peek().r#type != Type::OpenBrace {
            return self.parse_additive_expression();
        }

        self.advance();
        let mut properties: Vec<Property> = vec![];

        while self.not_eof() && self.peek().r#type != Type::CloseBrace {
            let key = self.advance();

            if key.r#type != Type::Identifier {
                panic!("expected identifier");
            }

            let key = match key.value {
                Value::String(value) => value,
                _ => panic!("expected string"),
            };

            match self.peek().r#type {
                Type::Comma => {
                    self.advance(); // consume comma
                    properties.push(Property { key, value: None });
                    continue;
                }
                Type::CloseBrace => {
                    properties.push(Property { key, value: None });
                    continue;
                }
                _ => (),
            }

            match self.advance().r#type {
                Type::Colon => (),
                _ => panic!("expected colon"),
            }

            let value = self.parse_expression();

            properties.push(Property {
                key,
                value: Some(value),
            });

            if self.peek().r#type != Type::CloseBrace {
                let comma = self.advance();

                if comma.r#type != Type::Comma {
                    panic!("expected comma");
                }
            }
        }

        let closing_brace = self.advance();

        if closing_brace.r#type != Type::CloseBrace {
            panic!("expected closing brace");
        }

        ASTExpression {
            kind: ASTExpressionKind::ObjectLiteral,
            body: ASTExpressionBody::Value(Value::Object(Object { properties })),
        }
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
