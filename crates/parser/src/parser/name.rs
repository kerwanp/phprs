use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::recognize,
    error::context,
    multi::many0_count,
    sequence::pair,
    Parser,
};

use crate::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name<'a>(pub &'a str);

impl<'a> Name<'a> {
    // TODO
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let parser = recognize(pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_")))),
        ));

        context("Name", parser).map(Self).parse(input)
    }
}
