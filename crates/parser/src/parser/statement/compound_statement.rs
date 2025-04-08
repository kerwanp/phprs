use nom::{bytes::complete::tag, error::context, sequence::delimited, Parser};

use crate::parser::{util::ws, Error};

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CompoundStatement<'a> {
    statements: Vec<Statement<'a>>,
}

impl<'a> CompoundStatement<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let parser = delimited(ws(tag("{")), Statement::parse_many, ws(tag("}")));
        context("CompoundStatement", parser)
            .map(|statements| Self { statements })
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn primary() {
        let variable = CompoundStatement::parse("{0b11;}");
        assert_matches!(variable, Ok(("", CompoundStatement { statements: _ })));
    }

    #[test]
    fn empty() {
        let variable = CompoundStatement::parse("{}");
        assert_matches!(variable, Ok(("", CompoundStatement { statements: _ })));
    }
}
