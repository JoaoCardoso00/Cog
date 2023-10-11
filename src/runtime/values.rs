pub enum ValueType {
    Null(NullValue),
    Number(NumberValue),
}

pub struct RuntimeValue {
    pub value_type: ValueType,
}

pub struct NullValue {
    pub r#type: String,
    pub value: String,
}

pub struct NumberValue {
    pub r#type: String,
    pub value: f64,
}
