use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char},
    combinator::{map, recognize},
    multi::many0_count,
    sequence::{pair, preceded},
    Parser,
};

use crate::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableName<'a>(&'a str);

impl<'a> VariableName<'a> {
    pub fn parser() -> impl Parser<&'a str, Self, Error<'a>> {
        let name = recognize(pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_")))),
        ));

        map(preceded(char('$'), name), Self)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn simple() {
        let variable = VariableName::parser().parse("$HelloWorld");
        assert_matches!(variable, Ok((_, VariableName("HelloWorld"))));
    }

    #[test]
    fn underscore() {
        let variable = VariableName::parser().parse("$_test_");
        assert_matches!(variable, Ok((_, VariableName("_test_"))));
    }

    #[test]
    fn numbers() {
        let variable = VariableName::parser().parse("$hey12_H");
        assert_matches!(variable, Ok((_, VariableName("hey12_H"))));
    }

    #[test]
    fn no_leading_number() {
        let variable = VariableName::parser().parse("$5test");
        assert_matches!(variable, Err(_));
    }
}
