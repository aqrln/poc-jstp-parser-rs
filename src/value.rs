use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum JstpValue {
    Undefined,
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JstpValue>),
    Object(HashMap<String, JstpValue>),
}
