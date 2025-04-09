use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::expressions::Expression;
use crate::parser::statements::Statement;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::name::variable_name::VariableName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PropertyElement<'a> {
    pub name: VariableName<'a>,
    pub initializer: Option<Expression<'a>>,
}

impl<'a> PropertyElement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let initializer = just(Token::Equals)
            .ignore_then(Expression::parser(statement_parser))
            .or_not();

        VariableName::parser()
            .then(initializer)
            .map(|(name, initializer)| Self { name, initializer })
    }

    pub fn list_parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Vec<Self>, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        // WARNING: Documentation seems really wrong https://phplang.org/spec/19-grammar.html#grammar-property-element
        Self::parser(statement_parser)
            .separated_by(just(Token::Comma))
            .collect()
    }
}
