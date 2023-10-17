use std::collections::HashMap;

use super::environment::Environment;

#[derive(Debug, Clone)]
pub(crate) enum ValueType {
    Null(NullValue),
    Number(NumberValue),
    Boolean(BooleanValue),
    Object(ObjectValue),
    NativeFunction(NativeFunctionValue),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueTypes {
    Null,
    Number,
    Boolean,
    Object,
    Function, // String,
}

#[derive(Debug, Clone)]
pub struct RuntimeValue {
    pub(crate) value_type: ValueType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NullValue {
    pub(crate) r#type: ValueTypes,
    pub(crate) value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NativeFunctionValue {
    pub(crate) r#type: ValueTypes,
    pub(crate) call: fn(Vec<RuntimeValue>, env: Environment) -> RuntimeValue,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumberValue {
    pub(crate) r#type: ValueTypes,
    pub(crate) value: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanValue {
    pub(crate) r#type: ValueTypes,
    pub(crate) value: bool,
}

#[derive(Debug, Clone)]
pub struct ObjectValue {
    pub(crate) r#type: ValueTypes,
    pub(crate) properties: HashMap<String, RuntimeValue>,
}
