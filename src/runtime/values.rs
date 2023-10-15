use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) enum ValueType {
    Null(NullValue),
    Number(NumberValue),
    Boolean(BooleanValue),
    Object(ObjectValue),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueTypes {
    Null,
    Number,
    Boolean,
    Object,
    // String,
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
