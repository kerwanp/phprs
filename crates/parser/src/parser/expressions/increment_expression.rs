use crate::parser::{variables::Variable, BoxedParser};
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InscrementExpression<'a>(Variable<'a>);

impl<'a> InscrementExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        choice((
            just(Token::PlusPlus).ignore_then(Variable::parser(expression_parser.clone())),
            Variable::parser(expression_parser).then_ignore(just(Token::PlusPlus)),
        ))
        .map(Self)
        .labelled("InscrementExpression")
    }
}
