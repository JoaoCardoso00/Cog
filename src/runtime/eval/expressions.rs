use core::panic;
use std::collections::HashMap;

use crate::{
    frontend::{
        lexer::lib::{Object, Value},
        parser::ast::{
            ASTExpression, ASTExpressionBody, ASTExpressionKind, ASTStatement, ASTStatementKind,
            BinaryExpression, CallExpression, VariableAssignment,
        },
    },
    helpers::build_null_runtime_value::build_null_runtime_value,
    runtime::{
        environment::Environment,
        values::{NumberValue, ObjectValue, RuntimeValue, ValueType, ValueTypes},
    },
};

use super::statements::{evaluate_expression, evaluate_statement};

pub fn evaluate_identifier_expression(identifier: String, env: &mut Environment) -> RuntimeValue {
    let val = env.peek_variable(identifier);
    val
}

pub fn evaluate_binary_expression(
    binary_exp: BinaryExpression,
    env: &mut Environment,
) -> RuntimeValue {
    let left_hand_side = binary_exp.left;
    let right_hand_side = binary_exp.right;

    let left_hand_side = evaluate_expression(*left_hand_side, env);
    let right_hand_side = evaluate_expression(*right_hand_side, env);

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

pub fn evaluate_assignment_expression(
    node: VariableAssignment,
    env: &mut Environment,
) -> RuntimeValue {
    if node.assignee.kind != ASTExpressionKind::Identifier {
        panic!("Invalid assignee type");
    }

    let variable_name = match node.assignee.body {
        ASTExpressionBody::Value(Value::String(value)) => value,
        _ => panic!("Invalid value type"),
    };

    let value_to_assign = evaluate_expression(*node.value, env);
    env.assign_variable(variable_name, value_to_assign)
}

pub fn evaluate_object_expression(obj: Object, env: &mut Environment) -> RuntimeValue {
    let mut object = ObjectValue {
        r#type: ValueTypes::Object,
        properties: HashMap::new(),
    };

    for property in obj.properties {
        let key = property.key;
        let value = property.value;

        let runtime_value: RuntimeValue = match value {
            Some(value) => {
                let value_statement = ASTStatement {
                    kind: ASTStatementKind::ExpressionStatement(ASTExpression {
                        kind: value.kind,
                        body: value.body,
                    }),
                };

                evaluate_statement(value_statement, env)
            }
            None => env.peek_variable(key.clone()),
        };

        object.properties.insert(key, runtime_value);
    }

    RuntimeValue {
        value_type: ValueType::Object(object),
    }
}

pub fn evaluate_call_expression(expression: CallExpression, env: &mut Environment) -> RuntimeValue {
    let args: Vec<RuntimeValue> = expression
        .arguments
        .iter()
        .map(|arg| {
            let arg_statement = ASTStatement {
                kind: ASTStatementKind::ExpressionStatement(ASTExpression {
                    kind: arg.kind.clone(),
                    body: arg.body.clone(),
                }),
            };

            evaluate_statement(arg_statement, env)
        })
        .collect();

    let caller_statement = ASTStatement {
        kind: ASTStatementKind::ExpressionStatement(ASTExpression {
            kind: expression.caller.kind,
            body: expression.caller.body,
        }),
    };

    let func = evaluate_statement(caller_statement, env);

    if matches!(func.value_type, ValueType::NativeFunction(_)) {
        let func = match func.value_type {
            ValueType::NativeFunction(func) => func,
            _ => panic!("Invalid value type"),
        };
        let result = (func.call)(args, env.clone());

        return result;
    }

    if matches!(func.value_type, ValueType::Function(_)) {
        let func = match func.value_type {
            ValueType::Function(func) => func,
            _ => panic!("Invalid value type"),
        };

        for (index, parameter) in func.parameters.iter().enumerate() {
            let arg = args.get(index).unwrap();
            env.declare_variable(parameter.clone(), arg.clone(), false);
        }

        let mut result: RuntimeValue = build_null_runtime_value();

        for statement in func.body {
            result = evaluate_statement(statement, env)
        }

        return result;
    }

    panic!("Invalid function type");
}
