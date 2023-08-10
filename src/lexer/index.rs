use super::token::Token;
use super::types::{Type, Value};
use crate::helpers::is_string::LiteralHelpers;

pub fn tokenize(source_code: String) -> Vec<Token<Value>> {
    let tokens: Vec<Token<Value>> = source_code
        .split(&[';', '\n', ' '][..])
        .filter(|string| !string.is_empty())
        .map(|token| match token {
            "let" => Token::<Value> {
                r#type: Type::Keyword,
                value: Value::String(String::from("let")),
            },
            "=" => Token::<Value> {
                r#type: Type::Operator,
                value: Value::String(String::from("=")),
            },
            "+" => Token::<Value> {
                r#type: Type::Operator,
                value: Value::String(String::from("+")),
            },
            "-" => Token::<Value> {
                r#type: Type::Operator,
                value: Value::String(String::from("-")),
            },
            "*" => Token::<Value> {
                r#type: Type::Operator,
                value: Value::String(String::from("*")),
            },
            "/" => Token::<Value> {
                r#type: Type::Operator,
                value: Value::String(String::from("/")),
            },
            number if number.parse::<f64>().is_ok() => Token::<Value> {
                r#type: Type::Number,
                value: Value::Number(
                    number
                        .parse::<f64>()
                        .expect("Error parsing numerical value"),
                ),
            },
            string if string.is_ascii() && string.is_string_literal() => Token::<Value> {
                r#type: Type::String,
                value: Value::String(String::from(string)),
            },
            variable if variable.is_ascii() => Token::<Value> {
                r#type: Type::Variable,
                value: Value::String(String::from(variable)),
            },
            _ => todo!(),
        })
        .collect();

    return tokens;
}
