use crate::runtime::values::{BooleanValue, RuntimeValue, ValueType, ValueTypes};

pub fn build_bool_runtime_value(boolean: bool) -> RuntimeValue {
    RuntimeValue {
        value_type: ValueType::Boolean(BooleanValue {
            r#type: ValueTypes::Boolean,
            value: boolean,
        }),
    }
}


