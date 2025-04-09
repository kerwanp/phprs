use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::name::variable_name::VariableName;
use crate::parser::expressions::Expression;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StaticVariableDeclaration<'a> {
    name: VariableName<'a>,
    initializer: Option<Expression<'a>>,
}

impl<'a> StaticVariableDeclaration<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let initializer = just(Token::Equals)
            .ignore_then(Expression::parser())
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
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::StaticKeyword)
            .ignore_then(
                StaticVariableDeclaration::parser()
                    .separated_by(just(Token::Comma))
                    .collect(),
            )
            .then_ignore(just(Token::Semicolon))
            .map(|variables| FunctionStaticDeclaration { variables })
            .labelled("StaticDeclaration")
    }
}
