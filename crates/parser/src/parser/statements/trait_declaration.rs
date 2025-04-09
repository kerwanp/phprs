use crate::parser::atoms::name::Name;
use crate::parser::interface::trait_member_declaration::TraitMemberDeclaration;
use crate::parser::BoxedParser;
use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TraitDeclaration<'a> {
    name: Name<'a>,
    body: Vec<TraitMemberDeclaration<'a>>,
}

impl<'a> TraitDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let body = just(Token::OpenBrace)
            .ignore_then(
                TraitMemberDeclaration::parser(statement_parser)
                    .repeated()
                    .collect(),
            )
            .then_ignore(just(Token::CloseBrace));

        just(Token::TraitKeyword)
            .ignore_then(Name::parser())
            .then(body)
            .map(|(name, body)| Self { name, body })
            .labelled("InterfaceDeclaration")
    }
}
