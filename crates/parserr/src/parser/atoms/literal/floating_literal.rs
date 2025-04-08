use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use phprs_lexer::Token;

// TODO: Separate in multiple struct
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FloatingLiteral<'a>(pub &'a str);

impl<'a> FloatingLiteral<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        select! {
            Token::FloatingLiteral(string) => Self(string),
        }
    }
}
