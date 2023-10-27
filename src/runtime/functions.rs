use crate::helpers::build_null_runtime_value::build_null_runtime_value;

use super::{
    environment::Environment,
    values::{NumberValue, RuntimeValue, ValueType, ValueTypes},
};

pub fn print(args: Vec<RuntimeValue>, _scope: Environment) -> RuntimeValue {
    for arg in args {
        let arg_value_type = arg.value_type;

        match arg_value_type {
            ValueType::Number(value) => {
                print!("{}", value.value);
            }
            ValueType::Boolean(value) => {
                print!("{}", value.value);
            }
            ValueType::String(value) => {
                print!("{}", value.value);
            }
            ValueType::Null(value) => {
                print!("{}", value.value);
            }
            ValueType::Object(value) => {
                print!("{:#?}", value);
            }
            ValueType::NativeFunction(value) => {
                print!("{:#?}", value);
            }
            ValueType::Function(value) => {
                print!("{:#?}", value);
            }
        }
    }

    build_null_runtime_value()
}

pub fn get_time(args: Vec<RuntimeValue>, _scope: Environment) -> RuntimeValue {
    if args.len() > 0 {
        panic!("get_time() takes no arguments");
    }

    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    RuntimeValue {
        value_type: ValueType::Number(NumberValue {
            r#type: ValueTypes::Number,
            value: time as f64,
        }),
    }
}
