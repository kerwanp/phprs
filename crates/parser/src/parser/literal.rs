pub mod binary_literal;
pub mod string_literal;

use binary_literal::BinaryLiteral;
use nom::{branch::alt, Parser};
use string_literal::StringLiteral;

use crate::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Literal<'a> {
    String(StringLiteral<'a>), // TODO
    Binary(BinaryLiteral<'a>),
}

impl<'a> Literal<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let string = StringLiteral::parse.map(Self::String);
        let binary = BinaryLiteral::parse.map(Self::Binary);
        alt((string, binary)).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn string() {
        let variable = Literal::parse(r#""Hello World!""#);
        assert_matches!(variable, Ok((_, Literal::String(_))));
    }

    #[test]
    fn binary() {
        let variable = Literal::parse("0b01001");
        assert_matches!(variable, Ok((_, Literal::Binary(_))));
    }
}
