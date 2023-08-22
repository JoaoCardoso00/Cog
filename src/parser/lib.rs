use core::panic;

use crate::lexer::lib::{Token, Type, Value};

use super::ast::{AssignmentExpression, Expression};

pub fn parse(tokens: Vec<Token>) -> Vec<Expression> {
    let mut expressions: Vec<Expression> = vec![];

    let test = build_assignment_expression(tokens);
    expressions.push(test);

    // let mut cursor = 0;

    // while cursor < tokens.len() {
    //     let current_token = tokens.get(cursor).unwrap();

    //     match current_token.r#type {
    //         Type::Keyword => match &current_token.value {
    //             Value::String(string) => match string.as_str() {

    //                 _ => todo!(),
    //             },
    //             _ => todo!(),
    //         },
    //         _ => todo!(),
    //     }
    // }

    expressions
}

fn build_assignment_expression(tokens: Vec<Token>) -> Expression {
    let mut token_iter = tokens.into_iter();

    let _ = token_iter.next(); // consume the "let"

    let variable_name = token_iter
        .next()
        .expect("Error parsing variable name for assignment expression");

    let variable_name = match variable_name.r#type {
        Type::Identifier => match variable_name.value {
            Value::String(string) => string,
            _ => panic!("Wrong type for assignment variable name"),
        },
        _ => panic!("Wrong type for assignment variable name"),
    };

    let _ = token_iter.next();

    let value = token_iter.next().expect("here").value;

    let assignment_expression = AssignmentExpression {
        identifier: variable_name,
        value,
    };

    let expression = Expression::Assignment(Box::new(assignment_expression));

    expression
}
