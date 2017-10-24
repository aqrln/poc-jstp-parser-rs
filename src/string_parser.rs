use std::{char, str};
use nom::digit;

#[derive(Debug, Copy, Clone)]
enum StrChunk<'a> {
    Slice(&'a str),
    Char(char),
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
                line_continuation | escape_sequence | single_quoted_chars
            )),
            |chunks: Vec<StrChunk>| {
                let total_len = chunks.iter().map(|chunk| match *chunk {
                    StrChunk::Slice(s) => s.len(),
                    StrChunk::Char(c) => c.len_utf8(),
                }).sum();

                let mut result = String::with_capacity(total_len);
                for chunk in chunks {
                    match chunk {
                        StrChunk::Slice(s) => result.push_str(s),
                        StrChunk::Char(c) => result.push(c),
                    }
                }

                result
            }
        ),
        tag_s!("'")
    )
);

named!(double_quoted<&str, String>, map!(tag_s!("\"\""), String::from));

named!(
    single_quoted_chars<&str, StrChunk>,
    map!(
        take_till_s!(|c| match c {
            '\'' | '\\' | '\n' | '\r' | '\u{2028}' | '\u{2029}' => true,
            _ => false,
        }),
        StrChunk::Slice
    )
);

named!(
    line_continuation<&str, StrChunk>,
    map!(
        preceded!(
            tag_s!("\\"),
            alt!(
                tag_s!("\n") |
                tag_s!("\r\n") |
                tag_s!("\u{2028}") |
                tag_s!("\u{2029}") |
                terminated!(tag_s!("\r"), peek!(not!(tag_s!("\n"))))
            )
        ),
        |_| StrChunk::Slice("")
    )
);

named!(
    escape_sequence<&str, StrChunk>,
    preceded!(
        tag_s!("\\"),
        alt!(
            terminated!(
                tag_s!("0"),
                peek!(not!(digit))
            ) => { |_| StrChunk::Char('\0') } |
            hex_escape_sequence => { |s| s } |
            unicode_escape_sequence => { |s| s } |
            char_escape_sequence => { |s| s }
        )
    )
);

named!(
    char_escape_sequence<&str, StrChunk>,
    map!(
        take_s!(1),
        |slice| {
            let c = slice.chars().next().unwrap();
            StrChunk::Char(match c {
                'b' => '\x08',
                'f' => '\x0c',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                'v' => '\x0b',
                _ => c,
            })
        }
    )
);

named!(hex_digit<&str, char>, one_of!("0123456789abcdefABCDEF"));

named!(
    hex_escape_sequence<&str, StrChunk>,
    do_parse!(
        tag_s!("x") >>
        first: hex_digit >>
        second: hex_digit >>
        ({
            let val = |digit: char| {
                digit.to_digit(16).unwrap() as u8
            };
            StrChunk::Char((val(first) * 16 + val(second)) as char)
        })
    )
);

named!(
    unicode_escape_sequence<&str, StrChunk>,
    preceded!(
        tag_s!("u"),
        map_opt!(
            alt!(es5_unicode_escape | es6_unicode_escape),
            |code| char::from_u32(code).map(|c| StrChunk::Char(c))
        )
    )
);

named!(
    es5_unicode_escape<&str, u32>,
    map!(
        tuple!(
            hex_digit,
            hex_digit,
            hex_digit,
            hex_digit
        ),
        |(a, b, c, d)| {
            let val = |c: char| c.to_digit(16).unwrap();
            val(a) * 0x1000 + val(b) * 0x100 + val(c) * 0x10 + val(d)
        }
    )
);

named!(
    es6_unicode_escape<&str, u32>,
    map!(
        delimited!(
            tag_s!("{"),
            many_m_n!(1, 6, hex_digit),
            tag_s!("}")
        ),
        |digits| {
            let len = digits.len();
            digits.into_iter()
                .zip((0..len).rev())
                .map(|(c, i)| {
                    c.to_digit(16).unwrap() * u32::pow(0x10, i as u32)
                })
                .sum()
        }
    )
);