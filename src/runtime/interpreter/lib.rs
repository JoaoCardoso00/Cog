use crate::{
    frontend::{
        lexer::lib::Value,
        parser::ast::{ASTExpressionBody, ASTExpressionKind, ASTStatement, ASTStatementKind},
    },
    runtime::values::{NullValue, NumberValue, RuntimeValue, ValueType},
};

pub fn evaluate(ast_node: ASTStatement) -> RuntimeValue {
    match ast_node.kind {
        ASTStatementKind::ExpressionStatement(expression) => match expression.kind {
            ASTExpressionKind::NumericLiteral => RuntimeValue {
                value_type: ValueType::Number(NumberValue {
                    r#type: "number".to_string(),
                    value: match expression.body {
                        ASTExpressionBody::Value(Value::Number(value)) => value,
                        _ => panic!("Invalid value type"),
                    },
                }),
            },
            _ => RuntimeValue {
                value_type: ValueType::Null(NullValue {
                    r#type: "null".to_string(),
                    value: "null".to_string(),
                }),
            },
        },
        _ => todo!(),
    };

    todo!()
}
