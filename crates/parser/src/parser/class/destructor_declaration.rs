use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::method_modifier::MethodModifier;
use crate::parser::statements::compound_statement::CompoundStatement;
use crate::parser::statements::Statement;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DestructorDeclaration<'a> {
    modifiers: Vec<MethodModifier>,
    reference: bool,
    body: Option<CompoundStatement<'a>>,
}

impl<'a> DestructorDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> BoxedParser<'a, I, Self>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let modifiers = MethodModifier::list_parser();
        let reference = just(Token::Ampersand).or_not().map(|n| n.is_some());

        let header = modifiers
            .then_ignore(just(Token::FunctionKeyword))
            .then(reference)
            .then_ignore(just(Token::DestructKeyword))
            .then_ignore(just(Token::OpenParen).then_ignore(just(Token::CloseParen)));

        let body = choice((
            just(Token::Semicolon).map(|_| None),
            CompoundStatement::parser(statement_parser).map(Some),
        ));

        header
            .then(body)
            .map(|((modifiers, reference), body)| Self {
                modifiers,
                reference,
                body,
            })
            .boxed()
    }
}
