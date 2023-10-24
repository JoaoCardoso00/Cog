use super::expressions::{
    evaluate_assignment_expression, evaluate_binary_expression, evaluate_call_expression,
    evaluate_identifier_expression, evaluate_object_expression,
};
use crate::{
    frontend::{
        lexer::lib::Value,
        parser::ast::{
            ASTExpression, ASTExpressionBody, ASTExpressionKind, ASTStatement, ASTStatementKind,
            VariableDeclaration, AST,
        },
    },
    helpers::build_null_runtime_value::build_null_runtime_value,
    runtime::{
        environment::Environment,
        values::{NumberValue, RuntimeValue, StringValue, ValueType, ValueTypes},
    },
};

pub fn evaluate_program(ast: AST, mut env: Environment) -> RuntimeValue {
    let mut last_evaluated = build_null_runtime_value();

    for statement in ast.statements {
        last_evaluated = evaluate_statement(statement, &mut env);
    }

    last_evaluated
}

pub fn evaluate_statement(ast_node: ASTStatement, env: &mut Environment) -> RuntimeValue {
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
            ASTExpressionKind::StringLiteral => RuntimeValue {
                value_type: ValueType::String(StringValue {
                    r#type: ValueTypes::String,
                    value: match expression.body {
                        ASTExpressionBody::Value(Value::String(value)) => value,
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
            ASTExpressionKind::AssignmentExpression => {
                let assignment_exp = match expression.body {
                    ASTExpressionBody::AssignmentExpressionBody(assignment_exp) => assignment_exp,
                    _ => panic!("Invalid expression type"),
                };

                evaluate_assignment_expression(assignment_exp, env)
            }
            ASTExpressionKind::ObjectLiteral => {
                let object = match expression.body {
                    ASTExpressionBody::Value(Value::Object(object)) => object,
                    _ => panic!("Invalid value type"),
                };

                evaluate_object_expression(object, env)
            }
            ASTExpressionKind::CallExpression => {
                let call_expression = match expression.body {
                    ASTExpressionBody::CallExpressionBody(call_expression) => call_expression,
                    _ => panic!("Invalid expression type"),
                };

                evaluate_call_expression(call_expression, env)
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
        ASTStatementKind::VariableDeclaration(variable_declaration) => {
            evaluate_variable_declaration(variable_declaration, env)
        }
        _ => panic!("This statement type is not supported yet: {:?}", ast_node),
    }
}

pub fn evaluate_variable_declaration(
    variable_declaration_statement: VariableDeclaration,
    mut env: &mut Environment,
) -> RuntimeValue {
    let variable_identifier = match variable_declaration_statement.identifier {
        Value::String(value) => value,
        _ => panic!("Invalid value type for variable identifier"),
    };

    let variable_value = match variable_declaration_statement.value {
        Some(value) => {
            let value_statement = ASTStatement {
                kind: ASTStatementKind::ExpressionStatement(ASTExpression {
                    kind: value.kind,
                    body: value.body,
                }),
            };

            evaluate_statement(value_statement, &mut env)
        }
        None => build_null_runtime_value(),
    };

    env.declare_variable(
        variable_identifier,
        variable_value,
        variable_declaration_statement.constant,
    )
}
