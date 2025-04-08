use nom::{
    bytes::complete::tag,
    error::context,
    sequence::{delimited, preceded},
    Parser,
};

use crate::parser::{util::ws, Error};

use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EmptyIntrinsic<'a> {
    expression: Expression<'a>,
}

impl<'a> EmptyIntrinsic<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let body = delimited(tag("("), Expression::parse, tag(")"));
        let parser = preceded(ws(tag("empty")), body).map(|expression| Self { expression });
        context("EmptyIntrinsic", parser).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn simple() {
        let variable = EmptyIntrinsic::parse(r#"empty("hello")"#);
        assert_matches!(variable, Ok(("", EmptyIntrinsic { expression: _ })));
    }
}
