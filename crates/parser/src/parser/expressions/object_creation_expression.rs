// TODO

use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use crate::parser::atoms::class_type_designator::ClassTypeDesignator;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::argument_expression::ArgumentExpression;
use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ObjectCreationExpression<'a> {
    Simple(ClassTypeDesignator<'a>, Vec<ArgumentExpression<'a>>),
    Class, // TODO
}

impl<'a> ObjectCreationExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        // TODO: Handle without parens
        let arguments = just(Token::OpenParen)
            .ignore_then(ArgumentExpression::list_parser(expression_parser.clone()))
            .then_ignore(just(Token::CloseParen));

        just(Token::NewKeyword)
            .ignore_then(ClassTypeDesignator::parser(expression_parser))
            .then(arguments)
            .map(|(designator, arguments)| ObjectCreationExpression::Simple(designator, arguments))
    }
}
