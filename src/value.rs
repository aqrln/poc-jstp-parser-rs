use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JsNumber {
    Integer(i64),
    Float(f64),
}

impl fmt::Display for JsNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsNumber::Integer(int) => write!(f, "{}", int),
            JsNumber::Float(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum JstpValue {
    Undefined,
    Null,
    Bool(bool),
    Number(JsNumber),
    String(String),
    Array(Vec<JstpValue>),
    Object(HashMap<String, JstpValue>),
}
