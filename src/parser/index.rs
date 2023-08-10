use crate::lexer::{
    token::Token,
    types::{Type, Value},
};

use super::ast::{AssignmentExpression, Expression};

pub fn parse(tokens: Vec<Token<Value>>) {
    build_assignment_expression(tokens)
}

fn build_assignment_expression(tokens: Vec<Token<Value>>) {
    let mut token_iter = tokens.into_iter();

    let _ = token_iter.next(); // consume the "let"

    todo!("Finish parsing for assignment functions");

    let variable_name = match token_iter
        .next()
        .expect("Error parsing variable name for assignment expression")
        .r#type
    {
        Type::Variable => match token_iter.nth(0).unwrap().value {
            Value::String(string) => println!("{:?}", string),
            _ => panic!("Wrong type for assignment variable name"),
        },
        _ => panic!("Wrong type for assignment variable name"),
    };

    dbg!(token_iter);

    let _ = token_iter.next();

    let value = token_iter.next().expect("here").value;

    let assignment_expression = AssignmentExpression {
        identifier: variable_name,
        value,
    };

    let expression = Expression::Assignment(Box::new(assignment_expression));

    println!("{:?}", expression);
}
