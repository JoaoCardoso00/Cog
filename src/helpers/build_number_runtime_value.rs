use crate::runtime::values::{NumberValue, RuntimeValue, ValueType, ValueTypes};

pub fn build_number_runtime_value(number: f64) -> RuntimeValue {
    RuntimeValue {
        value_type: ValueType::Number(NumberValue {
            r#type: ValueTypes::Number,
            value: number,
        }),
    }
}
