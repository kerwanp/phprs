use nom::{branch::alt, bytes::complete::tag, combinator::value, Parser};

use super::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RelativeScope {
    Self_,
    Parent,
    Static,
}

impl<'a> RelativeScope {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        alt((
            value(RelativeScope::Self_, tag("self")),
            value(RelativeScope::Parent, tag("parent")),
            value(RelativeScope::Static, tag("static")),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn self_() {
        let variable = RelativeScope::parse("self");
        assert_matches!(variable, Ok(("", RelativeScope::Self_)));
    }

    #[test]
    fn parent() {
        let variable = RelativeScope::parse("parent");
        assert_matches!(variable, Ok(("", RelativeScope::Parent)));
    }

    #[test]
    fn static_() {
        let variable = RelativeScope::parse("static");
        assert_matches!(variable, Ok(("", RelativeScope::Static)));
    }
}
