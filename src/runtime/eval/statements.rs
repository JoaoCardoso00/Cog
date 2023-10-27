use std::{cell::RefCell, rc::Rc};

use super::expressions::{
    evaluate_assignment_expression, evaluate_binary_expression, evaluate_call_expression,
    evaluate_identifier_expression, evaluate_object_expression,
};
use crate::{
    frontend::{
        lexer::lib::Value,
        parser::ast::{
            ASTExpression, ASTExpressionBody, ASTExpressionKind, ASTStatement, ASTStatementKind,
            FunctionDeclaration, VariableDeclaration, AST,
        },
    },
    helpers::build_null_runtime_value::build_null_runtime_value,
    runtime::{
        environment::Environment,
        values::{FunctionValue, NumberValue, RuntimeValue, StringValue, ValueType, ValueTypes},
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
        ASTStatementKind::ExpressionStatement(expression) => evaluate_expression(expression, env),
        ASTStatementKind::VariableDeclaration(variable_declaration) => {
            evaluate_variable_declaration(variable_declaration, env)
        }
        ASTStatementKind::FunctionDeclaration(function_declaration) => {
            evaluate_function_declaration(function_declaration, env)
        }
        _ => panic!("This statement type is not supported yet: {:?}", ast_node),
    }
}

pub fn evaluate_expression(expression: ASTExpression, env: &mut Environment) -> RuntimeValue {
    match expression.kind {
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
        Some(value) => evaluate_expression(value, &mut env),
        None => build_null_runtime_value(),
    };

    env.declare_variable(
        variable_identifier,
        variable_value,
        variable_declaration_statement.constant,
    )
}

pub fn evaluate_function_declaration(
    function_declaration: FunctionDeclaration,
    env: &mut Environment,
) -> RuntimeValue {
    // this should recieve a scope, but i don't know how to do it yet because lifetimes are a pain in the ass, so we can just have global scope for everything :)
    let func = RuntimeValue {
        value_type: ValueType::Function(FunctionValue {
            r#type: ValueTypes::Function,
            name: function_declaration.identifier.clone(),
            parameters: function_declaration.parameters,
            body: function_declaration.body,
            scope: Rc::new(RefCell::new(env.clone())),
        }),
    };

    let result = env.declare_variable(function_declaration.identifier, func, false);

    result
}
