use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use crate::parser::atoms::literal::Literal;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LiteralExpression<'a>(pub Literal<'a>);

impl<'a> LiteralExpression<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Literal::parser().map(Self).labelled("Literal expression")
    }
}
