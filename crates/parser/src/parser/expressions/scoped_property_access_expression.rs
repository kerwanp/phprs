use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use phprs_lexer::Token;

use crate::parser::atoms::scope_resolution_qualifier::ScopeResolutionQualifier;
use crate::parser::variables::simple::SimpleVariable;
use crate::parser::BoxedParser;

use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScopedPropertyAccessExpression<'a>(pub ScopeResolutionQualifier<'a>, SimpleVariable<'a>);

impl<'a> ScopedPropertyAccessExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        ScopeResolutionQualifier::parser()
            .then_ignore(just(Token::ColonColon))
            .then(SimpleVariable::parser(expression_parser))
            .map(|(scope, variable)| Self(scope, variable))
            .labelled("ScopePropertyAccessExpression")
    }
}
