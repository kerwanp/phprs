use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScalarType {
    Bool,
    Float,
    Int,
    String,
}

impl<'a> ScalarType {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        select! {
            Token::BoolReservedWord => Self::Bool,
            Token::FloatReservedWord => Self::Float,
            Token::IntReservedWord => Self::Int,
            Token::StringReservedWord => Self::String,
        }
    }
}
