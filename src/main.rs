use std::fs;

// use ariadne::{Label, Report, Source};

fn main() {
    let content = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    phprs_parser::parser::parse(&content);
}
