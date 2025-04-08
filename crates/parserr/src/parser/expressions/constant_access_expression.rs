use crate::parser::atoms::name::qualified_name::QualifiedName;
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstantAccessExpression<'a>(QualifiedName<'a>);

impl<'a> ConstantAccessExpression<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        QualifiedName::parser()
            .map(Self)
            .labelled("ConstantAccessExpression")
    }
}
