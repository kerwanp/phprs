use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::name::variable_name::VariableName;
use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StaticVariableDeclaration<'a> {
    name: VariableName<'a>,
    initializer: Option<Expression<'a>>,
}

impl<'a> StaticVariableDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        // TODO: Fix that
        let initializer = just(Token::Equals)
            .ignore_then(Expression::parser(statement_parser))
            .or_not();

        VariableName::parser()
            .then(initializer)
            .map(|(name, initializer)| StaticVariableDeclaration { name, initializer })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionStaticDeclaration<'a> {
    variables: Vec<StaticVariableDeclaration<'a>>,
}

impl<'a> FunctionStaticDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::StaticKeyword)
            .ignore_then(
                StaticVariableDeclaration::parser(statement_parser)
                    .separated_by(just(Token::Comma))
                    .collect(),
            )
            .then_ignore(just(Token::Semicolon))
            .map(|variables| FunctionStaticDeclaration { variables })
            .labelled("StaticDeclaration")
    }
}
