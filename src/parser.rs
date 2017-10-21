use std::str;

use nom::{alphanumeric, IResult};
use value::JstpValue;

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
    string<&str>,
    alt!(
        single_quote_string => { |s| s }
    )
);

named!(
    single_quote_string<&str>,
    delimited!(
        tag!("'"),
        map_res!(
            escaped!(call!(alphanumeric), '\\', one_of!("'n\\")),
            str::from_utf8
        ),
        tag!("'")
    )
);

named!(
    value<JstpValue>,
    ws!(alt!(
        undefined => { |_| JstpValue::Undefined } |
        null => { |_| JstpValue::Null } |
        boolean => { |b| JstpValue::Bool(b) } |
        array => { |v| JstpValue::Array(v) } |
        string => { |s| JstpValue::String(String::from(s)) }
    ))
);

pub fn parse(data: &[u8]) -> Option<(JstpValue, &[u8])> {
    match value(data) {
        IResult::Done(left, parsed) => Some((parsed, left)),
        _ => None,
    }
}
