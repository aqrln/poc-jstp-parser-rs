use nom::IResult;
use value::JstpValue;

use string_parser::string;

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
    array<&str, Vec<JstpValue>>,
    ws!(delimited!(
        tag_s!("["),
        many0!(alt!(
            tag_s!(",") => { |_| JstpValue::Undefined } |
            terminated!(value, tag_s!(",")) => { |v| v } |
            terminated!(value, peek!(tag_s!("]"))) => { |v| v }
        )),
        tag_s!("]")
    ))
);

named!(
    value<&str, JstpValue>,
    ws!(alt!(
        undefined => { |_| JstpValue::Undefined } |
        null => { |_| JstpValue::Null } |
        boolean => { |b| JstpValue::Bool(b) } |
        array => { |v| JstpValue::Array(v) } |
        string => { |s| JstpValue::String(s) }
    ))
);

pub fn parse(data: &str) -> Option<(JstpValue, &str)> {
    match value(data) {
        IResult::Done(left, parsed) => Some((parsed, left)),
        _ => None,
    }
}
