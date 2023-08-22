use crate::helpers::is_string::LiteralHelpers;
use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Type {
    Keyword,
    Identifier,
    Operator,
    Semi,
    Number,
    String,
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
}

#[derive(Debug)]
pub struct Token {
    pub r#type: Type,
    pub value: Value,
}


pub fn tokenize(input: &String) -> Result<Vec<Token>> {
    const NEW_LINE_CHARACTER: char = 0xA as char;
    let mut tokens: Vec<Token> = vec![];
    let mut cursor: usize = 0;
    let lines: Vec<&str> = input.lines().collect();

    while cursor < input.len() {
        let char = input.chars().nth(cursor).expect("internal error");

        match char {
            ' ' => {}
            NEW_LINE_CHARACTER => {}
            '=' => tokens.push(Token {
                r#type: Type::Operator,
                value: Value::String(String::from("=")),
            }),
            '+' => tokens.push(Token {
                r#type: Type::Operator,
                value: Value::String(String::from("+")),
            }),
            '-' => tokens.push(Token {
                r#type: Type::Operator,
                value: Value::String(String::from("-")),
            }),
            '*' => tokens.push(Token {
                r#type: Type::Operator,
                value: Value::String(String::from("*")),
            }),
            '/' => tokens.push(Token {
                r#type: Type::Operator,
                value: Value::String(String::from("/")),
            }),
            ';' => tokens.push(Token {
                r#type: Type::Semi,
                value: Value::String(String::from(";")),
            }),
            number if number.is_digit(10) => {
                let mut full_number = String::from(char);

                loop {
                    let next_char = input.chars().nth(cursor + 1).expect("internal error");

                    match next_char {
                        valid_char if valid_char.is_digit(10) => {
                            full_number.push(next_char);
                            cursor += 1;
                            continue;
                        }
                        ';' => break,
                        ' ' => break,
                        _ => bail!("Unable to read character at position {}", cursor + 1),
                    }
                }

                let full_number = full_number
                    .parse::<f64>()
                    .expect("error parsing number at position");

                tokens.push(Token {
                    r#type: Type::Number,
                    value: Value::Number(full_number),
                });
            }

            '\"' => {
                let mut full_statement = String::from(char);

                // get full statement before classifying it
                loop {
                    let next_char = input.chars().nth(cursor + 1);

                    match next_char {
                        Some(next_char) => match next_char {
                            ' ' => break,
                            valid_char if valid_char.is_ascii() => {
                                full_statement.push(next_char);
                                cursor += 1;

                                if valid_char == '"' {
                                    break;
                                }

                                continue;
                            }
                            _ => break,
                        },
                        None => break,
                    }
                }

                // dbg!(full_statement.as_str().is_string_literal());

                match full_statement {
                    string if string.is_string_literal() => tokens.push(Token {
                        r#type: Type::String,
                        value: Value::String(string),
                    }),

                    _ => bail!("failed to read string at position {}", cursor),
                }
            }

            char if char.is_ascii() => {
                let mut full_statement = String::from(char);

                // get full statement before classifying it
                loop {
                    let next_char = input.chars().nth(cursor + 1);

                    match next_char {
                        Some(next_char) => match next_char {
                            ' ' => break,
                            valid_char if valid_char.is_alphabetic() => {
                                full_statement.push(next_char);
                                cursor += 1;
                                continue;
                            }
                            _ => break,
                        },
                        None => break,
                    }
                }

                // statement can be either keyword, string or identifier
                match full_statement.as_str() {
                    "let" => tokens.push(Token {
                        r#type: Type::Keyword,
                        value: Value::String(String::from("let")),
                    }),
                    _ => tokens.push(Token {
                        r#type: Type::Identifier,
                        value: Value::String(full_statement),
                    }),
                }
            }
            _ => bail!("Unable to read character at position {}", cursor),
        };

        cursor += 1;
    }

    return Ok(tokens);
}