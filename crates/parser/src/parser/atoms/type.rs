pub mod return_type;
pub mod type_declaration;

use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Array,
    Callable,
    Iterable,
    ScalarType,    // TODO
    QualifiedName, // TODO
}

impl<'a> Type {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        select! {
            Token::ArrayKeyword => Type::Array,
            Token::CallableKeyword => Type::Callable,
            Token::IterableReservedWord => Type::Iterable,
        }
    }
}
