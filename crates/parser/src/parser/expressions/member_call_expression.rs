use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use crate::parser::atoms::member_name::MemberName;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::argument_expression::ArgumentExpression;
use super::dereferencable_expression::DereferencableExpression;
use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MemberCallExpression<'a> {
    expression: DereferencableExpression<'a>,
    member: MemberName<'a>,
    arguments: Vec<ArgumentExpression<'a>>,
}

impl<'a> MemberCallExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        DereferencableExpression::parser(expression_parser.clone())
            .then_ignore(just(Token::Arrow))
            .then(MemberName::parser(expression_parser.clone()))
            .then_ignore(just(Token::OpenParen))
            .then(ArgumentExpression::list_parser(expression_parser))
            .then_ignore(just(Token::CloseParen))
            .map(|((expression, member), arguments)| Self {
                expression,
                member,
                arguments,
            })
    }
}
