use crate::{
    frontend::{
        lexer::lib::Value,
        parser::ast::{ASTExpression, ASTStatement, ASTStatementKind, BinaryExpressionBody},
    },
    helpers::build_null_runtime_value::build_null_runtime_value,
    runtime::{
        environment::Environment,
        values::{NumberValue, RuntimeValue, ValueType, ValueTypes},
    },
};

use super::statements::evaluate_statement;

pub fn evaluate_identifier_expression(
    identifier: String,
    mut env: &mut Environment,
) -> RuntimeValue {
    let val = env.peek_variable(identifier);
    val
}

pub fn evaluate_binary_expression(
    binary_exp: BinaryExpressionBody,
    env: &mut Environment,
) -> RuntimeValue {
    let left_hand_side_statement = ASTStatement {
        kind: ASTStatementKind::ExpressionStatement(ASTExpression {
            kind: binary_exp.left.kind,
            body: binary_exp.left.body,
        }),
    };

    let right_hand_side_statement = ASTStatement {
        kind: ASTStatementKind::ExpressionStatement(ASTExpression {
            kind: binary_exp.right.kind,
            body: binary_exp.right.body,
        }),
    };

    let left_hand_side = evaluate_statement(left_hand_side_statement, env);
    let right_hand_side = evaluate_statement(right_hand_side_statement, env);

    if matches!(left_hand_side.value_type, ValueType::Number(_))
        && matches!(right_hand_side.value_type, ValueType::Number(_))
    {
        return RuntimeValue {
            value_type: ValueType::Number(evaluate_numeric_binary_expression(
                left_hand_side,
                right_hand_side,
                binary_exp.operator,
            )),
        };
    }

    build_null_runtime_value()
}

pub fn evaluate_numeric_binary_expression(
    left_hand_side: RuntimeValue,
    right_hand_side: RuntimeValue,
    operator: Value,
) -> NumberValue {
    let left_hand_side = match left_hand_side.value_type {
        ValueType::Number(value) => value,
        _ => panic!("Invalid value type"),
    };

    let right_hand_side = match right_hand_side.value_type {
        ValueType::Number(value) => value,
        _ => panic!("Invalid value type"),
    };

    let left_hand_side = left_hand_side.value;
    let right_hand_side = right_hand_side.value;

    let result = match operator {
        Value::String(value) => match value.as_str() {
            "+" => left_hand_side + right_hand_side,
            "-" => left_hand_side - right_hand_side,
            "*" => left_hand_side * right_hand_side,
            "/" => {
                if right_hand_side == 0.0 {
                    panic!("Division by zero");
                }

                left_hand_side / right_hand_side
            }
            "%" => left_hand_side % right_hand_side,
            _ => panic!("Invalid operator"),
        },
        _ => panic!("Invalid operator"),
    };

    NumberValue {
        r#type: ValueTypes::Number,
        value: result,
    }
}
