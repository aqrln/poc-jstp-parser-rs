#[macro_use]
extern crate nom;
extern crate ucd;

mod parser;
mod object_parser;
mod string_parser;
mod number_parser;
mod value;
mod str_chunk;

pub use value::{JsNumber, JstpValue};
pub use parser::parse;
