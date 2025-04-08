use nom::{bytes::complete::tag, combinator::cut, error::context, sequence::delimited, Parser};

use crate::parser::{expression::Expression, Error};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EchoStatement<'a> {
    expressions: Vec<Expression<'a>>,
}

impl<'a> EchoStatement<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let parser = delimited(tag("echo"), Expression::parse_many, cut(tag(";")))
            .map(|expressions| Self { expressions });

        context("EchoStatement", parser).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn primary() {
        let variable = EchoStatement::parse("echo 0b11;");
        assert_matches!(variable, Ok(("", EchoStatement { expressions: _ })));
    }
}
