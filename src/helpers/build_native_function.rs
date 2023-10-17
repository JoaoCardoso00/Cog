use crate::runtime::{
    environment::Environment,
    values::{NativeFunctionValue, RuntimeValue, ValueType, ValueTypes},
};

pub fn build_native_function(
    call: fn(Vec<RuntimeValue>, env: Environment) -> RuntimeValue,
) -> RuntimeValue {
    RuntimeValue {
        value_type: ValueType::NativeFunction(NativeFunctionValue {
            r#type: ValueTypes::Function,
            call,
        }),
    }
}
