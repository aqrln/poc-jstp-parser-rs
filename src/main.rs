extern crate jstp_parser;

use jstp_parser::*;

fn main() {
    println!("{:?}", parse("[, , null, 'hai', true, ]".as_bytes()));
}
