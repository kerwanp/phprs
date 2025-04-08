use crate::parser::atoms::{name::Name, scope_resolution_qualifier::ScopeResolutionQualifier};
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassConstantAccessExpression<'a> {
    scope: ScopeResolutionQualifier<'a>,
    name: Name<'a>,
}

impl<'a> ClassConstantAccessExpression<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        ScopeResolutionQualifier::parser()
            .then_ignore(just(Token::ColonColon))
            .then(Name::parser())
            .map(|(scope, name)| Self { scope, name })
            .labelled("ClassConstantAccessExpression")
    }
}
