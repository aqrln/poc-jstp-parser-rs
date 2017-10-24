use std::collections::HashMap;
use std::char;

use ucd::Codepoint;

use value::JsValue;
use string_parser::{string, es6_unicode_escape};
use number_parser::number;
use parser::value;
use str_chunk::StrChunk;

named!(
    pub object<&str, HashMap<String, JsValue>>,
    map!(
        ws!(delimited!(
            tag_s!("{"),
            separated_list!(tag_s!(","), key_value),
            tag_s!("}")
        )),
        |pairs| {
            pairs.into_iter().collect()
        }
    )
);

named!(
    key_value<&str, (String, JsValue)>,
    ws!(separated_pair!(
        key,
        tag_s!(":"),
        value
    ))
);

named!(
    key<&str, String>,
    alt!(
        string |
        ident |
        map!(number, |n| format!("{}", n))
    )
);

named!(
    ident<&str, String>,
    do_parse!(
        start: id_start >>
        cont: id_continue >>
        (aggregate_id_chunks(start, cont))
    )
);

fn aggregate_id_chunks(start: StrChunk, cont: Vec<StrChunk>) -> String {
    let mut total_len = cont.iter().map(|chunk| chunk.len()).sum();
    total_len += start.len();

    let mut result = String::with_capacity(total_len);

    {
        let mut push = |chunk| match chunk {
            StrChunk::Slice(s) => result.push_str(s),
            StrChunk::Char(c) => result.push(c),
        };

        push(start);

        for chunk in cont {
            push(chunk);
        }
    }

    result
}

macro_rules! id_chunk {
    ( $name:ident, $validator:expr ) => {
        named!(
            $name<&str, StrChunk>,
            alt!(
                map!(
                    take_while1_s!($validator),
                    StrChunk::Slice
                ) |
                map_opt!(
                    unicode_escape,
                    |c| if $validator(c) {
                        Some(StrChunk::Char(c))
                    } else {
                        None
                    }
                )
            )
        );
    }
}

id_chunk!(id_start, is_ecma262_id_start);
id_chunk!(id_continue_part, is_ecma262_id_continue);

named!(
    id_continue<&str, Vec<StrChunk>>,
    many0!(id_continue_part)
);

fn is_ecma262_id_start(c: char) -> bool {
    c.is_id_start() || c == '$' || c == '_'
}

fn is_ecma262_id_continue(c: char) -> bool {
    c.is_id_continue() || c == '$' || c == '_' || c == '\u{200C}'
        || c == '\u{200D}'
}

named!(
    unicode_escape<&str, char>,
    map_opt!(
        preceded!(tag_s!("\\u"), es6_unicode_escape),
        |code| char::from_u32(code)
    )
);
