pub mod enum_type;
pub mod return_type;
pub mod scalar_type;
pub mod type_declaration;

use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;
use scalar_type::ScalarType;

use super::name::qualified_name::QualifiedName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type<'a> {
    Array,
    Callable,
    Iterable,
    Mixed,
    ScalarType(ScalarType),
    QualifiedName(QualifiedName<'a>),
}

impl<'a> Type<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let keywords = select! {
            Token::ArrayKeyword => Type::Array,
            Token::MixedReservedWord => Type::Mixed,
            Token::CallableKeyword => Type::Callable,
            Token::IterableReservedWord => Type::Iterable,
        };

        let qualified = QualifiedName::parser().map(Self::QualifiedName);
        let scalar = ScalarType::parser().map(Self::ScalarType);

        choice((keywords, scalar, qualified))
    }
}
