use nom::{branch::alt, bytes::complete::tag, combinator::value, Parser};

use super::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScalarType {
    Bool,
    Float,
    Int,
    String,
}

impl<'a> ScalarType {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        alt((
            value(Self::Bool, tag("bool")),
            value(Self::Float, tag("float")),
            value(Self::Int, tag("int")),
            value(Self::String, tag("string")),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn bool() {
        let variable = ScalarType::parse("bool");
        assert_matches!(variable, Ok(("", ScalarType::Bool)));
    }

    #[test]
    fn float() {
        let variable = ScalarType::parse("float");
        assert_matches!(variable, Ok(("", ScalarType::Float)));
    }

    #[test]
    fn int() {
        let variable = ScalarType::parse("int");
        assert_matches!(variable, Ok(("", ScalarType::Int)));
    }

    #[test]
    fn string() {
        let variable = ScalarType::parse("string");
        assert_matches!(variable, Ok(("", ScalarType::String)));
    }
}
