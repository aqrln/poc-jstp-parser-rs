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
        decimal_with_fractional |
        decimal_without_integer
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
    decimal_integer<&str, ()>,
    alt!(
        map!(tag_s!("0"), |_| ()) |
        map!(preceded!(
            one_of!("123456789"),
            many0!(one_of!("0123456789"))
        ), |_| ())
    )
);

named!(
    decimal_digits<&str, Vec<char>>,
    many1!(one_of!("0123456789"))
);

named!(
    exponent_part<&str, Vec<char>>,
    preceded!(
        one_of!("eE"),
        preceded!(opt!(one_of!("+-")), decimal_digits)
    )
);
