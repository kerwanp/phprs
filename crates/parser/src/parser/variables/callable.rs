use crate::parser::{
    expressions::{member_call_expression::MemberCallExpression, Expression},
    BoxedParser,
};
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

use super::simple::SimpleVariable;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CallableVariable<'a> {
    Simple(SimpleVariable<'a>),
    SubscriptExpression,                            // TODO
    MemberCallExpression(MemberCallExpression<'a>), // TODO
    ScopedCallExpression,                           // TODO
    FunctionCallExpression,                         // TODO
}

impl<'a> CallableVariable<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let simple =
            SimpleVariable::parser(expression_parser.clone()).map(CallableVariable::Simple);
        let member =
            MemberCallExpression::parser(expression_parser).map(Self::MemberCallExpression);

        choice((member, simple)).labelled("Callable variable")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<CallableVariable, ()> {
        let token_stream = tokenize(src);

        CallableVariable::parser(Expression::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"$test"#);
        assert!(matches!(res, Ok(CallableVariable::Simple(_))));
    }

    #[test]
    fn nested() {
        let res = parse(r#"$$$test"#);
        assert!(matches!(
            res,
            Ok(CallableVariable::Simple(SimpleVariable::Simple(_)))
        ));
    }
}
