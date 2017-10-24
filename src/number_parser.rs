use value::JsNumber;

named!(
    pub number<&str, JsNumber>,
    alt!(
        map!(integer, JsNumber::Integer) |
        map!(float, JsNumber::Float)
    )
);

named!(
    integer<&str, i64>,
    map!(tag_s!("10"), |_| 10)
);

named!(
    float<&str, f64>,
    map!(tag_s!("12.3"), |_| 12.3)
);
