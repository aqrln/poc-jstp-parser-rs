#[macro_use]
extern crate nom;

mod parser;
mod object_parser;
mod string_parser;
mod value;

pub use value::JstpValue;
pub use parser::parse;
