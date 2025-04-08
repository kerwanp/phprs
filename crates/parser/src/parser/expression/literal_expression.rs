use nom::Parser;

use crate::parser::{literal::Literal, Error};
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LiteralExpression<'a>(Literal<'a>);

impl<'a> LiteralExpression<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        Literal::parse.map(Self).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn simple() {
        let variable = LiteralExpression::parse("0b11");
        assert_matches!(variable, Ok(("", LiteralExpression(_))));
    }
}
