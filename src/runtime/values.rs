#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
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
    pub value_type: ValueType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NullValue {
    pub r#type: ValueTypes,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumberValue {
    pub r#type: ValueTypes,
    pub value: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanValue {
    pub r#type: ValueTypes,
    pub value: bool,
}
