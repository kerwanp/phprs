use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::name::qualified_name::QualifiedName;
use crate::parser::atoms::name::variable_name::VariableName;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::compound_statement::CompoundStatement;
use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CatchClause<'a> {
    catches: Vec<QualifiedName<'a>>,
    variable_name: VariableName<'a>,
    body: CompoundStatement<'a>,
}

impl<'a> CatchClause<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::CatchKeyword)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(
                QualifiedName::parser()
                    .separated_by(just(Token::Bar))
                    .collect(),
            )
            .then(VariableName::parser())
            .then_ignore(just(Token::CloseParen))
            .then(CompoundStatement::parser(statement_parser))
            .map(|((catches, variable_name), body)| Self {
                catches,
                variable_name,
                body,
            })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TryStatement<'a> {
    body: CompoundStatement<'a>,
    catch_clauses: Vec<CatchClause<'a>>,
    finally_clause: Option<CompoundStatement<'a>>,
}

impl<'a> TryStatement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let catch_clauses = CatchClause::parser(statement_parser.clone())
            .repeated()
            .collect();

        let finally_clause = just(Token::FinallyKeyword)
            .ignore_then(CompoundStatement::parser(statement_parser.clone()))
            .or_not();

        just(Token::TryKeyword)
            .ignore_then(CompoundStatement::parser(statement_parser))
            .then(catch_clauses)
            .then(finally_clause)
            .map(|((body, catch_clauses), finally_clause)| Self {
                body,
                catch_clauses,
                finally_clause,
            })
    }
}
