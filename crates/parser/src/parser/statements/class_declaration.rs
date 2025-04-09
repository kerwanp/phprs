use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::class_modifier::ClassModifier;
use crate::parser::atoms::name::qualified_name::QualifiedName;
use crate::parser::atoms::name::Name;
use crate::parser::class::class_member_declaration::ClassMemberDeclaration;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassDeclaration<'a> {
    name: Name<'a>,
    modifier: Option<ClassModifier>,
    extends: Option<QualifiedName<'a>>,
    implements: Vec<QualifiedName<'a>>,
    body: Vec<ClassMemberDeclaration<'a>>,
}

impl<'a> ClassDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let modifier = ClassModifier::parser().or_not();
        let extends = just(Token::ExtendsKeyword)
            .ignore_then(QualifiedName::parser())
            .or_not();

        let implements = just(Token::ImplementsKeyword)
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
                ClassMemberDeclaration::parser(statement_parser)
                    .repeated()
                    .collect(),
            )
            .then_ignore(just(Token::CloseBrace));

        modifier
            .then_ignore(just(Token::ClassKeyword))
            .then(Name::parser())
            .then(extends)
            .then(implements)
            .then(body)
            .map(|((((modifier, name), extends), implements), body)| Self {
                name,
                modifier,
                extends,
                implements,
                body,
            })
            .labelled("ClassDeclaration")
    }
}
