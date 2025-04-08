use crate::parser::BoxedParser;
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequireExpression<'a> {
    expression: Expression<'a>,
}

impl<'a> RequireExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::RequireKeyword)
            .ignore_then(expression_parser)
            .map(|expression| Self { expression })
            .labelled("Require expression")
    }
}
