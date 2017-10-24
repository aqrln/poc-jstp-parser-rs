extern crate jstp_parser;

use jstp_parser::*;

fn main() {
    println!(
        "{:?}",
        parse("[, , null, 'hai\\n\\\n second \\x0a', true, ]")
    );
}
