use nom::IResult;
use value::JstpValue;

use string_parser::string;

named!(undefined, tag!("undefined"));

named!(null, tag!("null"));

named!(
    boolean<bool>,
    alt!(
        tag!("true") => { |_| true } |
        tag!("false") => { |_| false }
    )
);

named!(
    array<Vec<JstpValue>>,
    ws!(delimited!(
        tag!("["),
        many0!(alt!(
            tag!(",") => { |_| JstpValue::Undefined } |
            terminated!(value, tag!(",")) => { |v| v } |
            terminated!(value, peek!(tag!("]"))) => { |v| v }
        )),
        tag!("]")
    ))
);

named!(
    value<JstpValue>,
    ws!(alt!(
        undefined => { |_| JstpValue::Undefined } |
        null => { |_| JstpValue::Null } |
        boolean => { |b| JstpValue::Bool(b) } |
        array => { |v| JstpValue::Array(v) } |
        string => { |s| JstpValue::String(s) }
    ))
);

pub fn parse(data: &[u8]) -> Option<(JstpValue, &[u8])> {
    match value(data) {
        IResult::Done(left, parsed) => Some((parsed, left)),
        _ => None,
    }
}
