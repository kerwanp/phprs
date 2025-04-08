use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use phprs_lexer::Token;

// TODO: Separate in multiple struct
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BinaryLiteral<'a>(pub &'a str);

impl<'a> BinaryLiteral<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        select! {
            Token::BinaryLiteral(string) => Self(string),
        }
    }
}
