use crate::{
    frontend::{
        lexer::lib::Value,
        parser::ast::{
            ASTExpression, ASTExpressionBody, ASTExpressionKind, ASTStatement, ASTStatementKind,
            BinaryExpressionBody, AST,
        },
    },
    helpers::build_null_runtime_value::build_null_runtime_value,
    runtime::{
        environment::Environment,
        values::{NumberValue, RuntimeValue, ValueType, ValueTypes},
    },
};

pub fn evaluate(ast: AST, env: Environment) -> RuntimeValue {
    match ast.kind {
        "Program" => evaluate_program(ast, env),
        _ => panic!("Unknown AST kind: {}", ast.kind),
    }
}

fn evaluate_program(ast: AST, env: Environment) -> RuntimeValue {
    let mut last_evaluated = build_null_runtime_value();

    for statement in ast.statements {
        last_evaluated = evaluate_statement(statement, env.clone());
    }

    last_evaluated
}

fn evaluate_statement(ast_node: ASTStatement, env: Environment) -> RuntimeValue {
    match ast_node.kind {
        ASTStatementKind::ExpressionStatement(expression) => match expression.kind {
            ASTExpressionKind::NumericLiteral => RuntimeValue {
                value_type: ValueType::Number(NumberValue {
                    r#type: ValueTypes::Number,
                    value: match expression.body {
                        ASTExpressionBody::Value(Value::Number(value)) => value,
                        _ => panic!("Invalid value type"),
                    },
                }),
            },
            ASTExpressionKind::BinaryExpression => {
                let binary_exp = match expression.body {
                    ASTExpressionBody::BinaryExpressionBody(binary_exp) => binary_exp,
                    _ => panic!("Invalid expression type"),
                };

                evaluate_binary_expression(binary_exp, env)
            }

            ASTExpressionKind::Identifier => {
                let identifier = match expression.body {
                    ASTExpressionBody::Value(Value::String(value)) => value,
                    _ => panic!("Invalid value type"),
                };

                evaluate_identifier_expression(identifier, env)
            }

            _ => panic!(
                "This expression type is not supported yet: {:?}",
                expression
            ),
        },
        _ => panic!("This statement type is not supported yet: {:?}", ast_node),
    }
}

fn evaluate_identifier_expression(identifier: String, mut env: Environment) -> RuntimeValue {
    let val = env.peek_variable(identifier);
    val
}

fn evaluate_binary_expression(binary_exp: BinaryExpressionBody, env: Environment) -> RuntimeValue {
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

    let left_hand_side = evaluate_statement(left_hand_side_statement, env.clone());
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

fn evaluate_numeric_binary_expression(
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
