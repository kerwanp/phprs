use nom::{combinator::map, Parser};

use crate::Error;

use super::simple_variable::SimpleVariable;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CallableVariable<'a> {
    SimpleVariable(SimpleVariable<'a>),
    SubscriptExpression,    // TODO
    MemberCallExpression,   // TODO
    ScopedCallExpression,   // TODO
    FunctionCallExpression, // TODO
}

impl<'a> CallableVariable<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let mut simple_variable = map(SimpleVariable::parse, Self::SimpleVariable);
        simple_variable.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn simple_variable() {
        let variable = CallableVariable::parse("$test");
        assert_matches!(variable, Ok((_, CallableVariable::SimpleVariable(_))));
    }
}
