use nom::{branch::alt, character::complete::char, combinator::map, sequence::preceded, Parser};

use crate::Error;

use super::variable_name::VariableName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SimpleVariable<'a> {
    VariableName(VariableName<'a>),
    SimpleVariable(Box<SimpleVariable<'a>>),
    Epxression, // TODO
}

impl<'a> SimpleVariable<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let variable_name = map(VariableName::parser(), Self::VariableName);

        let simple_variable = map(preceded(char('$'), Self::parse), |v| {
            Self::SimpleVariable(Box::new(v))
        });

        alt((variable_name, simple_variable)).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn variable_name() {
        let variable = SimpleVariable::parse("$test");
        assert_matches!(variable, Ok((_, SimpleVariable::VariableName(_))));
    }

    #[test]
    fn simple_variable() {
        let variable = SimpleVariable::parse("$$test");
        assert_matches!(variable, Ok((_, SimpleVariable::SimpleVariable(_))));
    }

    #[test]
    fn nested_simple_variable() {
        let variable = SimpleVariable::parse("$$$test");
        assert_matches!(variable, Ok((_, SimpleVariable::SimpleVariable(_))));
    }
}
