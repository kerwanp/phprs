use crate::parser::atoms::name::qualified_name::QualifiedName;
use crate::parser::atoms::name::Name;
use crate::parser::interface::interface_member_declaration::InterfaceMemberDeclaration;
use crate::parser::BoxedParser;
use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InterfaceDeclaration<'a> {
    name: Name<'a>,
    extends: Vec<QualifiedName<'a>>,
    body: Vec<InterfaceMemberDeclaration<'a>>,
}

impl<'a> InterfaceDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let extends = just(Token::ExtendsKeyword)
            .ignore_then(
                QualifiedName::parser()
                    .separated_by(just(Token::Comma))
                    .at_least(1)
                    .collect(),
            )
            .or_not()
            .map(|names| names.unwrap_or_default());

        let body = just(Token::OpenBrace)
            .ignore_then(
                InterfaceMemberDeclaration::parser(statement_parser)
                    .repeated()
                    .collect(),
            )
            .then_ignore(just(Token::CloseBrace));

        just(Token::InterfaceKeyword)
            .ignore_then(Name::parser())
            .then(extends)
            .then(body)
            .map(|((name, extends), body)| Self {
                name,
                extends,
                body,
            })
            .labelled("InterfaceDeclaration")
    }
}
