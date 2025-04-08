use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::literal::Literal;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeclareDirective<'a> {
    Ticks(Literal<'a>),
    Encoding(Literal<'a>),
    StrictTypes(Literal<'a>),
}

impl<'a> DeclareDirective<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let ticks = just(Token::Name("ticks"))
            .then_ignore(just(Token::Equals))
            .ignore_then(Literal::parser())
            .map(DeclareDirective::Ticks);
        let encoding = just(Token::Name("encoding"))
            .then_ignore(just(Token::Equals))
            .ignore_then(Literal::parser())
            .map(DeclareDirective::Encoding);
        let strict_types = just(Token::Name("strict_types"))
            .then_ignore(just(Token::Equals))
            .ignore_then(Literal::parser())
            .map(DeclareDirective::StrictTypes);

        ticks.or(encoding).or(strict_types)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeclareStatement<'a> {
    directive: DeclareDirective<'a>,
    statements: Vec<Statement<'a>>,
}

impl<'a> DeclareStatement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let start = just(Token::DeclareKeyword)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(DeclareDirective::parser())
            .then_ignore(just(Token::CloseParen));

        let with_statement = statement_parser.clone().map(|s| vec![s]);
        let with_keywords = just(Token::Colon)
            .ignore_then(Statement::list_parser(statement_parser))
            .then_ignore(just(Token::EndDeclareKeyword))
            .then_ignore(just(Token::Semicolon));

        let empty = just(Token::Semicolon).map(|_| vec![]);

        start
            .then(with_statement.or(with_keywords).or(empty))
            .map(|(directive, statements)| DeclareStatement {
                directive,
                statements,
            })
            .labelled("DeclareStatement")
    }
}
