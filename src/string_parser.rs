use std::str;
use nom::{digit, hex_digit};

enum StrChunk<'a> {
    Slice(&'a str),
    Char(char)
}

named!(
    pub string<&str, String>,
    alt!(single_quoted | double_quoted)
);

named!(
    single_quoted<&str, String>,
    delimited!(
        tag_s!("'"),
        map!(
            many1!(alt!(
                single_quoted_chars | line_continuation | escape_sequence
            )),
            |slices: Vec<&str>| {
                let total_len = slices.iter().map(|v| v.len()).sum();
                let mut result = String::with_capacity(total_len);
                for slice in slices.into_iter() {
                    result.push_str(slice);
                }
                result
            }
        ),
        tag_s!("'")
    )
);

named!(double_quoted<&str, String>, map!(tag_s!("\"\""), String::from));

named!(
    single_quoted_chars<&str, &str>,
    take_till_s!(|c| match c {
        '\'' | '\\' | '\n' | '\r' | '\u{2028}' | '\u{2029}' => true,
        _ => false,
    })
);

named!(
    line_continuation<&str, &'static str>,
    map!(
        preceded!(
            tag_s!("\\"),
            alt!(
                tag_s!("\n") | tag_s!("\r\n") | tag_s!("\u{2028}") | tag_s!("\u{2029}")
                    | terminated!(tag_s!("\r"), peek!(not!(tag_s!("\n"))))
            )
        ),
        |_| ""
    )
);

named!(
    escape_sequence<&str, &str>,
    preceded!(
        tag_s!("\\"),
        alt!(
            terminated!(tag_s!("0"), peek!(not!(digit))) => { |_| "\0" } |
            hex_escape_sequence => { |s| s } |
            unicode_escape_sequence => { |s| s } |
            char_escape_sequence => { |s| s }
        )
    )
);

named!(
    char_escape_sequence<&str, &str>,
    tag_s!("n")
);

named!(
    hex_escape_sequence<&str, &str>,
    do_parse!(
        tag_s!("x") >>
        first: hex_digit >>
        second: hex_digit >>
        ("")
    )
);

named!(
    unicode_escape_sequence<&str, &str>,
    tag_s!("n")
);
