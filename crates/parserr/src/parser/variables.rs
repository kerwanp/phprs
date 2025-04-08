pub mod callable;
pub mod simple;

use crate::parser::BoxedParser;
use callable::CallableVariable;
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

use super::expressions::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Variable<'a> {
    Callable(CallableVariable<'a>),
    ScopedPropertryAccessExpression,
    MemberAccessExpression,
}

impl<'a> Variable<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let callable = CallableVariable::parser(expression_parser).map(Variable::Callable);

        callable.labelled("Variable")
    }

    pub fn list_parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Vec<Self>, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Self::parser(expression_parser)
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .at_least(1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<Variable, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        Variable::parser(Expression::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"$test"#);
        // assert!(matches!(res, Ok(Variable::Callable(_))));
    }
}
