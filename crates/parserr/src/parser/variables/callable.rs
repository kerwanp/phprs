use crate::parser::{expressions::Expression, BoxedParser};
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

use super::simple::SimpleVariable;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CallableVariable<'a> {
    Simple(SimpleVariable<'a>),
}

impl<'a> CallableVariable<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let simple = SimpleVariable::parser(expression_parser).map(CallableVariable::Simple);

        simple.labelled("Callable variable")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        // let res = parse(r#"$test"#);
        // assert!(matches!(res, Ok(CallableVariable::Simple(_))));
    }
}
