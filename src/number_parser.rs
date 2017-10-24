use value::JsNumber;

named!(
    pub number<&str, JsNumber>,
    do_parse!(
        sign: opt!(one_of!("+-")) >>
        num: alt!(
            binary_literal |
            octal_literal |
            hexadecimal_literal |
            decimal_literal
        ) >>
        (match sign {
            Some(sign_char) if sign_char == '-' => {
                match num {
                    JsNumber::Integer(int) => JsNumber::Integer(-int),
                    JsNumber::Float(flt) => JsNumber::Float(-flt),
                }
            }
            _ => num
        })
    )
);

named!(
    decimal_literal<&str, JsNumber>,
    alt!(
        decimal_integer_literal |
        decimal_with_fractional |
        decimal_without_integer
    )
);

named!(
    decimal_integer_literal<&str, JsNumber>,
    map_res!(
        recognize!(terminated!(
            decimal_integer,
            peek!(not!(alt!(char!('.') | one_of!("eE"))))
        )),
        |input: &str| input.parse::<i64>().map(JsNumber::Integer)
    )
);

named!(
    decimal_with_fractional<&str, JsNumber>,
    map_res!(
        recognize!(tuple!(
            decimal_integer,
            opt!(preceded!(tag_s!("."), decimal_digits)),
            opt!(exponent_part)
        )),
        |input: &str| input.parse::<f64>().map(JsNumber::Float)
    )
);

named!(
    decimal_without_integer<&str, JsNumber>,
    map_res!(
        recognize!(tuple!(
            preceded!(tag_s!("."), decimal_digits),
            opt!(exponent_part)
        )),
        |input: &str| input.parse::<f64>().map(JsNumber::Float)
    )
);

named!(
    decimal_integer<&str, &str>,
    alt!(
        tag_s!("0") |
        preceded!(one_of!("123456789"), take_while_s!(is_dec_digit))
    )
);

named!(
    decimal_digits<&str, &str>,
    take_while1_s!(is_dec_digit)
);

named!(
    exponent_part<&str, &str>,
    preceded!(
        one_of!("eE"),
        preceded!(opt!(one_of!("+-")), decimal_digits)
    )
);

macro_rules! integer_parsers {
    ( $(
        $name:ident => (
            $radix:expr,
            $lower:expr,
            $upper:expr,
            $predicate:expr
        )
    ),* ) => {
        $(named!{
            $name<&str, JsNumber>,
            preceded!(
                alt!(tag_s!($lower) | tag_s!($upper)),
                map_res!(
                    recognize!(take_while1_s!($predicate)),
                    |input: &str| {
                        i64::from_str_radix(input, $radix)
                            .map(JsNumber::Integer)
                    }
                )
            )
        })*
    }
}

integer_parsers!(
    binary_literal => (2, "0b", "0B", is_bin_digit),
    octal_literal => (8, "0o", "0O", is_oct_digit),
    hexadecimal_literal => (16, "0x", "0X", is_hex_digit)
);

fn is_bin_digit(c: char) -> bool {
    return c == '0' || c == '1';
}

fn is_oct_digit(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7'  => true,
        _ => false,
    }
}

fn is_dec_digit(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

fn is_hex_digit(c: char) -> bool {
    if is_dec_digit(c) {
        return true;
    }
    match c {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' => true,
        'A' | 'B' | 'C' | 'D' | 'E' | 'F' => true,
        _ => false,
    }
}
