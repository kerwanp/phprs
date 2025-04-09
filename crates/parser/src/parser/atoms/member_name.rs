use crate::parser::BoxedParser;
use crate::parser::{expressions::Expression, variables::simple::SimpleVariable};
use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use super::name::Name;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MemberName<'a> {
    Name(Name<'a>),
    SimpleVariable(SimpleVariable<'a>),
    Expression(Expression<'a>),
}

impl<'a> MemberName<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        choice((
            Name::parser().map(Self::Name),
            SimpleVariable::parser(expression_parser.clone()).map(Self::SimpleVariable),
            expression_parser.map(Self::Expression),
        ))
    }
}
