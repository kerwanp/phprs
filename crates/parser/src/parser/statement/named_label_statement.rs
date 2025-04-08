use nom::{bytes::complete::tag, error::context, sequence::terminated, Parser};

use crate::parser::{name::Name, util::ws, Error};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamedLabelStatement<'a> {
    name: Name<'a>,
}

impl<'a> NamedLabelStatement<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let parser = terminated(Name::parse, ws(tag(":")));
        context("NamedLabelStatement", parser)
            .map(|name| Self { name })
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn simple() {
        let variable = NamedLabelStatement::parse("test:");
        assert_matches!(
            variable,
            Ok(("", NamedLabelStatement { name: Name("test") }))
        );
    }
}
