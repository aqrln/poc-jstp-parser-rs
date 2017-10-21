extern crate jstp_parser;

use jstp_parser::*;

fn main() {
    println!("{:?}", parse("[null, , true, 'hello\\nkitty']".as_bytes()));
}
