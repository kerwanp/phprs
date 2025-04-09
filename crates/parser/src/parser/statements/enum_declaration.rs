use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::name::Name;
use crate::parser::atoms::r#type::enum_type::EnumType;
use crate::parser::class::enum_member_declaration::EnumMemberDeclaration;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnumDeclaration<'a> {
    name: Name<'a>,
    body: Vec<EnumMemberDeclaration<'a>>,
    r#type: Option<EnumType>,
}

// TODO: Handle `enum Suite: string implements Colorful`
impl<'a> EnumDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let body = just(Token::OpenBrace)
            .ignore_then(
                EnumMemberDeclaration::parser(statement_parser)
                    .repeated()
                    .collect(),
            )
            .then_ignore(just(Token::CloseBrace));

        let r#type = just(Token::Colon).ignore_then(EnumType::parser()).or_not();

        just(Token::EnumKeyword)
            .ignore_then(Name::parser())
            .then(r#type)
            .then(body)
            .map(|((name, r#type), body)| Self { name, body, r#type })
            .labelled("EnumDeclaration")
    }
}
