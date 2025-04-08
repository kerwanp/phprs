use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, value},
    error::context,
    sequence::terminated,
    Parser,
};

use crate::parser::{expression::Expression, Error};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExpressionStatement<'a>(Option<Expression<'a>>);

impl<'a> ExpressionStatement<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let parser = terminated(Expression::parse, cut(char(';'))).map(|e| Self(Some(e)));
        let empty = value(Self(None), tag(";"));
        context("ExpressionStatement", alt((parser, empty))).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn primary() {
        let variable = ExpressionStatement::parse("0b11;");
        assert_matches!(variable, Ok(("", ExpressionStatement(_))));
    }

    #[test]
    fn empty() {
        let variable = ExpressionStatement::parse(";");
        assert_matches!(variable, Ok(("", ExpressionStatement(_))));
    }
}
