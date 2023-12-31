use std::io::Write;

use crate::{frontend::parser::ast::ASTExpression, helpers::is_string::LiteralHelpers};

#[derive(Debug, Copy, Clone, PartialEq)]

pub enum Type {
    // identifier
    Identifier,

    // variable declaration
    Let,
    Const,
    Fn,
    For,
    In,
    If,
    Else,
    While,

    // operators
    Operator,          // +, -, *, /
    Interval,          // ..
    InclusiveInterval, // ..=
    GreaterThan,       // >
    LessThan,          // <
    GreaterEqual,      // >=
    LessEqual,         // <=
    Not,               // !
    NotEqual,          // !=
    OpenParen,         // (
    CloseParen,        // )
    Comma,             // ,
    Colon,             // :
    OpenBrace,         // {
    CloseBrace,        // }
    OpenBracket,       // [
    CloseBracket,      // ]
    Semi,              // ;
    Dot,               // .
    Equals,            // =

    // values
    Number,
    String,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Object(Object),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub(crate) properties: Vec<Property>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub(crate) key: String,
    pub(crate) value: Option<ASTExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub(crate) r#type: Type,
    pub(crate) value: Value,
}

pub fn tokenize(input: &String) -> Result<Vec<Token>, String> {
    const NEW_LINE_CHARACTER: char = 0xA as char;
    let operators = vec![
        '+',
        '-',
        '*',
        '/',
        '%',
        ':',
        '=',
        '(',
        ')',
        '{',
        '}',
        ';',
        ',',
        '[',
        ']',
        '.',
        '>',
        '<',
        '!',
        NEW_LINE_CHARACTER,
    ];
    let mut tokens: Vec<Token> = vec![];
    let mut cursor: usize = 0;

    while cursor < input.len() {
        let char = input.chars().nth(cursor).expect("internal error");

        match char {
            ' ' => {}
            NEW_LINE_CHARACTER => {}
            '=' => tokens.push(Token {
                r#type: Type::Equals,
                value: Value::String(String::from("=")),
            }),
            '+' => tokens.push(Token {
                r#type: Type::Operator,
                value: Value::String(String::from("+")),
            }),
            ',' => tokens.push(Token {
                r#type: Type::Comma,
                value: Value::String(String::from(",")),
            }),
            '!' => {
                let next_char = input.chars().nth(cursor + 1);

                if next_char == Some('=') {
                    tokens.push(Token {
                        r#type: Type::NotEqual,
                        value: Value::String(String::from("!=")),
                    });
                    cursor += 1;
                } else {
                    tokens.push(Token {
                        r#type: Type::Not,
                        value: Value::String(String::from("!")),
                    })
                }
            }
            ':' => tokens.push(Token {
                r#type: Type::Colon,
                value: Value::String(String::from(":")),
            }),
            '>' | '<' => {
                let next_char = input.chars().nth(cursor + 1);

                if next_char == Some('=') {
                    match char {
                        '>' => tokens.push(Token {
                            r#type: Type::GreaterEqual,
                            value: Value::String(String::from(">=")),
                        }),
                        '<' => tokens.push(Token {
                            r#type: Type::LessEqual,
                            value: Value::String(String::from("<=")),
                        }),
                        _ => panic!("internal error"),
                    }

                    cursor += 1;
                } else {
                    match char {
                        '>' => tokens.push(Token {
                            r#type: Type::GreaterThan,
                            value: Value::String(String::from(">")),
                        }),
                        '<' => tokens.push(Token {
                            r#type: Type::LessThan,
                            value: Value::String(String::from("<")),
                        }),
                        _ => panic!("internal error"),
                    }
                }
            }
            '{' => tokens.push(Token {
                r#type: Type::OpenBrace,
                value: Value::String(String::from("{")),
            }),
            '}' => tokens.push(Token {
                r#type: Type::CloseBrace,
                value: Value::String(String::from("}")),
            }),
            '.' => {
                let next_char = input.chars().nth(cursor + 1);

                if next_char == Some('.') {
                    let inclusive = input.chars().nth(cursor + 2);

                    if inclusive == Some('=') {
                        tokens.push(Token {
                            r#type: Type::InclusiveInterval,
                            value: Value::String(String::from("..=")),
                        });
                        cursor += 2;
                    } else {
                        tokens.push(Token {
                            r#type: Type::Interval,
                            value: Value::String(String::from("..")),
                        });
                        cursor += 1;
                    }
                } else {
                    tokens.push(Token {
                        r#type: Type::Dot,
                        value: Value::String(String::from(".")),
                    });
                }
            }
            '[' => tokens.push(Token {
                r#type: Type::OpenBracket,
                value: Value::String(String::from("[")),
            }),
            ']' => tokens.push(Token {
                r#type: Type::CloseBracket,
                value: Value::String(String::from("]")),
            }),
            '%' => tokens.push(Token {
                r#type: Type::Operator,
                value: Value::String(String::from("%")),
            }),
            '(' => tokens.push(Token {
                r#type: Type::OpenParen,
                value: Value::String(String::from("(")),
            }),
            ')' => tokens.push(Token {
                r#type: Type::CloseParen,
                value: Value::String(String::from(")")),
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
                    let next_char = input.chars().nth(cursor + 1);

                    if next_char.is_none() {
                        break;
                    }

                    match next_char.unwrap() {
                        valid_char if valid_char.is_digit(10) => {
                            full_number.push(next_char.unwrap());
                            cursor += 1;
                            continue;
                        }
                        ';' => break,
                        ' ' => break,
                        operator if operators.contains(&operator) => break,
                        _ => panic!("Unable to read character at position {}", cursor + 1),
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
                            '\\' => break,
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

                match full_statement {
                    string if string.is_string_literal() => tokens.push(Token {
                        r#type: Type::String,
                        value: Value::String(string),
                    }),

                    _ => panic!("failed to read string at position {}", cursor),
                }
            }

            char if char.is_alphabetic() => {
                let mut full_statement = String::from(char);

                // get full statement before classifying it
                loop {
                    let next_char = input.chars().nth(cursor + 1);

                    match next_char {
                        Some(next_char) => match next_char {
                            ' ' => break,
                            operator if operators.contains(&operator) => break,
                            valid_char
                                if valid_char.is_ascii() && !operators.contains(&valid_char) =>
                            {
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
                        r#type: Type::Let,
                        value: Value::String(String::from("let")),
                    }),
                    "const" => tokens.push(Token {
                        r#type: Type::Const,
                        value: Value::String(String::from("const")),
                    }),
                    "for" => tokens.push(Token {
                        r#type: Type::For,
                        value: Value::String(String::from("for")),
                    }),
                    "in" => tokens.push(Token {
                        r#type: Type::In,
                        value: Value::String(String::from("in")),
                    }),
                    "if" => tokens.push(Token {
                        r#type: Type::If,
                        value: Value::String(String::from("if")),
                    }),
                    "else" => tokens.push(Token {
                        r#type: Type::Else,
                        value: Value::String(String::from("else")),
                    }),
                    "fn" => tokens.push(Token {
                        r#type: Type::Fn,
                        value: Value::String(String::from("fn")),
                    }),
                    "while" => tokens.push(Token {
                        r#type: Type::While,
                        value: Value::String(String::from("while")),
                    }),
                    _ => tokens.push(Token {
                        r#type: Type::Identifier,
                        value: Value::String(full_statement),
                    }),
                }
            }
            _ => panic!("Unable to read character at position {}", cursor),
        };

        cursor += 1;
    }

    tokens.push(Token {
        r#type: Type::EOF,
        value: Value::String(String::from("\0")),
    });

    return Ok(tokens);
}
