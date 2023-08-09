use super::types::Type;

#[derive(Debug)]
pub struct Token<TValue> {
    pub r#type: Type,
    pub value: TValue,
}