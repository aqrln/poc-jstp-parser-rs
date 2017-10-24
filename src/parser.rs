use nom::IResult;
use value::JsValue;

use object_parser::object;
use string_parser::string;
use number_parser::number;

named!(undefined<&str, &str>, tag_s!("undefined"));

named!(null<&str, &str>, tag_s!("null"));

named!(
    boolean<&str, bool>,
    alt!(
        tag_s!("true") => { |_| true } |
        tag_s!("false") => { |_| false }
    )
);

named!(
    array<&str, Vec<JsValue>>,
    ws!(delimited!(
        tag_s!("["),
        many0!(alt!(
            tag_s!(",") => { |_| JsValue::Undefined } |
            terminated!(value, tag_s!(",")) => { |v| v } |
            terminated!(value, peek!(tag_s!("]"))) => { |v| v }
        )),
        tag_s!("]")
    ))
);

named!(
    pub value<&str, JsValue>,
    ws!(alt!(
        undefined => { |_| JsValue::Undefined } |
        null => { |_| JsValue::Null } |
        boolean => { |b| JsValue::Bool(b) } |
        array => { |v| JsValue::Array(v) } |
        string => { |s| JsValue::String(s) } |
        number => { |n| JsValue::Number(n) } |
        object => { |o| JsValue::Object(o) }
    ))
);

pub fn parse(data: &str) -> Option<(JsValue, &str)> {
    match value(data) {
        IResult::Done(left, parsed) => Some((parsed, left)),
        _ => None,
    }
}
