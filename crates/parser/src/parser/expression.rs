use nom::{bytes::complete::tag, multi::separated_list1, Parser};
use primary_expression::PrimaryExpression;

use crate::Error;

pub mod empty_intrinsic;
pub mod eval_intrinsic;
pub mod literal_expression;
pub mod primary_expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression<'a> {
    Primary(Box<PrimaryExpression<'a>>),
}

impl<'a> Expression<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let mut primary = PrimaryExpression::parse.map(|v| Self::Primary(Box::new(v)));

        primary.parse(input)
    }

    pub fn parse_many(input: &'a str) -> nom::IResult<&'a str, Vec<Self>, Error<'a>> {
        separated_list1(tag(","), Self::parse).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn primary() {
        let variable = Expression::parse("0b11");
        assert_matches!(variable, Ok(("", Expression::Primary(_))));
    }
}
