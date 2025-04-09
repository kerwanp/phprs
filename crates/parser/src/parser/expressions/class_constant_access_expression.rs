use crate::parser::atoms::{name::Name, scope_resolution_qualifier::ScopeResolutionQualifier};
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ClassConstantAccessExpression<'a> {
    Name {
        scope: ScopeResolutionQualifier<'a>,
        name: Name<'a>,
    },

    // TODO: Might be in its own struct.
    // ::class is not referenced in the grammar
    Class {
        scope: ScopeResolutionQualifier<'a>,
    },
}

impl<'a> ClassConstantAccessExpression<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let name = ScopeResolutionQualifier::parser()
            .then_ignore(just(Token::ColonColon))
            .then(Name::parser())
            .map(|(scope, name)| Self::Name { scope, name });

        let class = ScopeResolutionQualifier::parser()
            .then_ignore(just(Token::ColonColon))
            .then_ignore(just(Token::ClassKeyword))
            .map(|scope| Self::Class { scope });

        choice((name, class)).labelled("ClassConstantAccessExpression")
    }
}
