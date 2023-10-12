#[derive(Debug, PartialEq)]
pub enum ValueType {
    Null(NullValue),
    Number(NumberValue),
}

#[derive(Debug)]
pub struct RuntimeValue {
    pub value_type: ValueType,
}

#[derive(Debug, PartialEq)]
pub struct NullValue {
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct NumberValue {
    pub r#type: String,
    pub value: f64,
}
