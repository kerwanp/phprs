use nom::{
    branch::alt, bytes::complete::tag, character::complete::one_of, combinator::recognize,
    multi::many1, sequence::preceded, Parser,
};

use crate::parser::{util::ws, Error};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BinaryLiteral<'a>(&'a str);

impl<'a> BinaryLiteral<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let prefix = alt((tag("0b"), tag("0B")));
        let binary = recognize(many1(one_of("01")));
        ws(preceded(prefix, binary)).map(Self).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn lowercase() {
        let variable = BinaryLiteral::parse("0b01001");
        assert_matches!(variable, Ok(("", BinaryLiteral("01001"))));
    }

    #[test]
    fn uppercase() {
        let variable = BinaryLiteral::parse("0B01001");
        assert_matches!(variable, Ok(("", BinaryLiteral("01001"))));
    }

    #[test]
    fn escape() {
        let variable = BinaryLiteral::parse("  0B01001  ");
        assert_matches!(variable, Ok(("", BinaryLiteral("01001"))));
    }
}
