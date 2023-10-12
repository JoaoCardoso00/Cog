use crate::{
    frontend::{
        lexer::lib::Value,
        parser::ast::{
            ASTExpression, ASTExpressionBody, ASTExpressionKind, ASTStatement, ASTStatementKind,
            BinaryExpressionBody, AST,
        },
    },
    runtime::values::{NullValue, NumberValue, RuntimeValue, ValueType},
};

pub fn evaluate(ast: AST) -> RuntimeValue {
    match ast.kind {
        "Program" => evaluate_program(ast),
        _ => panic!("Unknown AST kind: {}", ast.kind),
    }
}

fn evaluate_program(ast: AST) -> RuntimeValue {
    let mut last_evaluated = RuntimeValue {
        value_type: ValueType::Null(NullValue {
            r#type: "Null".to_string(),
            value: "null".to_string(),
        }),
    };

    for statement in ast.statements {
        last_evaluated = evaluate_statement(statement);
    }

    last_evaluated
}

fn evaluate_statement(ast_node: ASTStatement) -> RuntimeValue {
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
            ASTExpressionKind::NullLiteral => RuntimeValue {
                value_type: ValueType::Null(NullValue {
                    r#type: "null".to_string(),
                    value: "null".to_string(),
                }),
            },
            ASTExpressionKind::BinaryExpression => {
                let binary_exp = match expression.body {
                    ASTExpressionBody::BinaryExpressionBody(binary_exp) => binary_exp,
                    _ => panic!("Invalid expression type"),
                };

                evaluate_binary_expression(binary_exp)
            }

            _ => panic!(
                "This expression type is not supported yet: {:?}",
                expression
            ),
        },
        _ => todo!(),
    }
}

fn evaluate_binary_expression(binary_exp: BinaryExpressionBody) -> RuntimeValue {
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

    let left_hand_side = evaluate_statement(left_hand_side_statement);
    let right_hand_side = evaluate_statement(right_hand_side_statement);

    if matches!(left_hand_side.value_type, ValueType::Number(_))
        || matches!(right_hand_side.value_type, ValueType::Number(_))
    {
        return RuntimeValue {
            value_type: ValueType::Number(evaluate_numeric_binary_expression(
                left_hand_side,
                right_hand_side,
                binary_exp.operator,
            )),
        };
    }

    RuntimeValue {
        value_type: ValueType::Null(NullValue {
            r#type: "null".to_string(),
            value: "null".to_string(),
        }),
    }
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
            //TODO: Handle division by zero
            "/" => left_hand_side / right_hand_side,
            "%" => left_hand_side % right_hand_side,
            _ => panic!("Invalid operator"),
        },
        _ => panic!("Invalid operator"),
    };

    NumberValue {
        r#type: "number".to_string(),
        value: result,
    }
}
