use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use crate::parser::BoxedParser;

use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayElement<'a> {
    key: Option<Expression<'a>>,
    value: Expression<'a>,
    reference: bool,
}

impl<'a> ArrayElement<'a> {
    // TODO: Could be `element-key =>(opt) &(opt) element-value`
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let value = just(Token::Ampersand)
            .or_not()
            .map(|t| t.is_some())
            .then(expression_parser.clone());

        let with_key = expression_parser
            .clone()
            .then_ignore(just(Token::DoubleArrow))
            .then(value.clone())
            .map(|(key, (reference, value))| Self {
                key: Some(key),
                value,
                reference,
            });

        choice((
            with_key,
            value.map(|(reference, value)| Self {
                key: None,
                reference,
                value,
            }),
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayCreationExpression<'a> {
    elements: Vec<ArrayElement<'a>>,
}

impl<'a> ArrayCreationExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let elements = ArrayElement::parser(expression_parser)
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .collect()
            .map(|elements| Self { elements });

        let with_keyword = just(Token::ArrayKeyword)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(elements.clone())
            .then_ignore(just(Token::CloseParen));

        let without_keyword = just(Token::OpenBracket)
            .ignore_then(elements)
            .then_ignore(just(Token::CloseBracket));

        choice((with_keyword, without_keyword))
    }
}
