#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ValueType {
    Null(NullValue),
    Number(NumberValue),
    Boolean(BooleanValue),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueTypes {
    Null,
    Number,
    Boolean,
    String,
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
