use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::name::qualified_name::QualifiedName;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TraitUseClause<'a> {
    names: Vec<QualifiedName<'a>>,
}

impl<'a> TraitUseClause<'a> {
    pub fn parser<I>(// statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let names = QualifiedName::parser()
            .separated_by(just(Token::Comma))
            .collect();
        // TODO trait-use-specification

        just(Token::UseKeyword)
            .ignore_then(names)
            .then_ignore(just(Token::Semicolon))
            .map(|names| Self { names })
    }
}
