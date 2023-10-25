use crate::runtime::{values::{NullValue, RuntimeValue, ValueType, ValueTypes}};

pub fn build_null_runtime_value() -> RuntimeValue {
    RuntimeValue {
        value_type: ValueType::Null(NullValue {
            r#type: ValueTypes::Null,
            value: "null".to_string(),
        }),
    }
}

