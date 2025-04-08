#![feature(assert_matches)]

pub mod parser;

use nom::{combinator::eof, sequence::terminated, Parser};
use parser::script::Script;

type Error<'a> = nom::error::VerboseError<&'a str>;

pub fn parse(input: &str) -> nom::IResult<&str, Script, Error<'_>> {
    terminated(Script::parse, eof).parse(input)
}
