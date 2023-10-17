use core::panic;

use crate::{
    frontend::lexer::lib::{Token, Type, Value},
    frontend::{
        lexer::lib::{tokenize, Object, Property},
        parser::ast::{
            BinaryExpression, CallExpression, MemberExpression, VariableAssignment,
            VariableDeclaration,
        },
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
            _ => panic!(
                "expected \";\" at variable declaration, found {:?}",
                self.peek()
            ),
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
                body: ASTExpressionBody::BinaryExpressionBody(BinaryExpression {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }),
            };
        }

        left
    }

    fn parse_multiplicative_expression(&mut self) -> ASTExpression {
        let mut left = self.parse_call_member_expression();

        while self.peek().value == Value::String("*".to_string())
            || self.peek().value == Value::String("/".to_string())
            || self.peek().value == Value::String("%".to_string())
        {
            let operator = self.advance().value;

            let right = self.parse_primary_expression();

            left = ASTExpression {
                kind: ASTExpressionKind::BinaryExpression,
                body: ASTExpressionBody::BinaryExpressionBody(BinaryExpression {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }),
            };
        }

        left
    }

    fn parse_call_member_expression(&mut self) -> ASTExpression {
        let member = self.parse_member_expression();

        if self.peek().r#type == Type::OpenParen {
            return self.parse_call_expression(member);
        }

        member
    }

    fn parse_call_expression(&mut self, caller: ASTExpression) -> ASTExpression {
        let mut call_expression = ASTExpression {
            kind: ASTExpressionKind::CallExpression,
            body: ASTExpressionBody::CallExpressionBody(CallExpression {
                arguments: self.parse_arguments(),
                caller: Box::new(caller),
            }),
        };

        if self.peek().r#type == Type::OpenParen {
            call_expression = self.parse_call_expression(call_expression);
        }

        call_expression
    }

    fn parse_arguments(&mut self) -> Vec<ASTExpression> {
        let open_paren = self.advance();

        if open_paren.r#type != Type::OpenParen {
            panic!("expected opening parenthesis");
        }

        let arguments = match self.peek().r#type {
            Type::CloseParen => vec![],
            _ => self.parse_arguments_list(),
        };

        let closing_paren = self.advance();

        if closing_paren.r#type != Type::CloseParen {
            panic!("missing closing parenthesis");
        }

        arguments
    }

    fn parse_arguments_list(&mut self) -> Vec<ASTExpression> {
        let mut arguments = vec![self.parse_assignment_expression()];

        while self.peek().r#type == Type::Comma && self.not_eof() {
            self.advance();
            arguments.push(self.parse_assignment_expression());
        }

        arguments
    }

    fn parse_member_expression(&mut self) -> ASTExpression {
        let mut object = self.parse_primary_expression();

        while self.peek().r#type == Type::Dot || self.peek().r#type == Type::OpenBracket {
            let operator = self.advance();
            let property: ASTExpression;
            let computed: bool;

            match operator.r#type {
                Type::Dot => {
                    property = self.parse_primary_expression();
                    computed = false;

                    if property.kind != ASTExpressionKind::Identifier {
                        panic!("expected identifier");
                    }
                }

                _ => {
                    computed = true;
                    property = self.parse_expression();

                    let closing_bracket = self.advance();

                    if closing_bracket.r#type != Type::CloseBracket {
                        panic!("expected closing bracket");
                    }
                }
            }

            object = ASTExpression {
                kind: ASTExpressionKind::MemberExpression,
                body: ASTExpressionBody::MemberExpressionBody(MemberExpression {
                    object: Box::new(object),
                    property: Box::new(property),
                    computed,
                }),
            };
        }

        object
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
