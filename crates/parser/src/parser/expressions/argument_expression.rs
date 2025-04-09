use crate::parser::BoxedParser;
use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArgumentExpression<'a> {
    variadic: bool,
    expression: Expression<'a>,
}

impl<'a> ArgumentExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let variadic = just(Token::DotDotDot).or_not().map(|a| a.is_some());

        variadic
            .then(expression_parser)
            .map(|(variadic, expression)| Self {
                variadic,
                expression,
            })
    }

    pub fn list_parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Vec<Self>, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Self::parser(expression_parser)
            .separated_by(just(Token::Comma))
            .collect()
    }
}
