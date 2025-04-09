use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use super::name::qualified_name::QualifiedName;
use super::relative_scope::RelativeScope;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScopeResolutionQualifier<'a> {
    RelativeScope(RelativeScope),
    QualifiedName(QualifiedName<'a>),
    DereferencableExpression, // TODO
}

impl<'a> ScopeResolutionQualifier<'a> {
    // TODO: Could be tokens
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let relative_scope = RelativeScope::parser().map(Self::RelativeScope);
        let qualified_name = QualifiedName::parser().map(Self::QualifiedName);

        choice((relative_scope, qualified_name))
    }
}

// TODO: tests
