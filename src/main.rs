extern crate jstp_parser;

use jstp_parser::*;

fn main() {
    println!(
        "{:?}\n{:?}\n{:?}",
        parse("[, , null, '', '\\na', 'hai\\n\\\n second \\x0a', true, ]"),
        parse("'nya \\u0439 \\u{1F496}'"),
        parse("'i love \\'-strings'"),
    );
}
