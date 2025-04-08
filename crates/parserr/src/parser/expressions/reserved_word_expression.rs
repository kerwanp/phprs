use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ReservedWordExpression {
    True,
    False,
    Null,
}

impl<'a> ReservedWordExpression {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        select! {
            Token::TrueReservedWord => Self::True,
            Token::FalseReservedWord => Self::False,
            Token::NullReservedWord => Self::Null,
        }
        .labelled("ReservedWord expression")
    }
}
