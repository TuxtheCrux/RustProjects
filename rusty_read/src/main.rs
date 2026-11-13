use std::env;
use crate::arg_parser;
fn main() {
    let args: Vec<String> = env::args().collect();
    arg_parser::parser();
}
