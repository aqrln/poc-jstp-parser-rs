extern crate jstp_parser;

use jstp_parser::*;

fn main() {
    println!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        parse("[, , null, '', '\\na', 'hai\\n\\\n second \\x0a', true, ]"),
        parse("'nya \\u0439 \\u{1F496}'"),
        parse("'i love \\'-strings'"),
        parse("[, , null, \"\", \"\\na\", \"hai\\n\\\n second \\x0a\" ]"),
        parse("\"i love \\\"-strings too\""),
        parse("{ 'nya': [ 'kawaii' ], 'a': true }"),
        parse("{ nya: 'kawaii', _: true, a18: '32' }"),
        parse("[1, 2, 3, 10.3, -8, 7e-3]"),
    );
}
