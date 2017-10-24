use value::JsNumber;

named!(
    pub number<&str, JsNumber>,
    do_parse!(
        sign: opt!(one_of!("+-")) >>
        num: alt!(
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
        preceded!(one_of!("123456789"), take_while_s!(is_decimal_digit))
    )
);

named!(
    decimal_digits<&str, &str>,
    take_while1_s!(is_decimal_digit)
);

named!(
    exponent_part<&str, &str>,
    preceded!(
        one_of!("eE"),
        preceded!(opt!(one_of!("+-")), decimal_digits)
    )
);

fn is_decimal_digit(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}
