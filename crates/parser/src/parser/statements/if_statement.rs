use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ElseClause<'a>(Statement<'a>);

impl<'a> ElseClause<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::ElseKeyword)
            .ignore_then(statement_parser)
            .labelled("ElseClause")
            .map(ElseClause)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ElseIfClause<'a> {
    expression: Expression<'a>,
    statement: Statement<'a>,
}

impl<'a> ElseIfClause<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        // TODO: Heavily used, move outside for caching?
        let expr =
            Expression::parser().delimited_by(just(Token::OpenParen), just(Token::CloseParen));

        just(Token::ElseIfKeyword)
            .ignore_then(expr)
            .then(statement_parser)
            .map(|(expression, statement)| ElseIfClause {
                expression,
                statement,
            })
            .labelled("ElseIf clause")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IfStatement<'a> {
    expression: Expression<'a>,
    statement: Statement<'a>,
    elseif_clauses: Vec<ElseIfClause<'a>>,
    else_clause: Option<ElseClause<'a>>,
}

impl<'a> IfStatement<'a> {
    // TODO: Manage with keywords endif
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let expr =
            Expression::parser().delimited_by(just(Token::OpenParen), just(Token::CloseParen));

        let if_clause = just(Token::IfKeyword)
            .ignore_then(expr.clone())
            .then(statement_parser.clone())
            .labelled("If clause");

        if_clause
            .then(
                ElseIfClause::parser(statement_parser.clone())
                    .repeated()
                    .collect(),
            )
            .then(ElseClause::parser(statement_parser).or_not())
            .map(
                |(((expression, statement), elseif_clauses), else_clause)| IfStatement {
                    expression,
                    statement,
                    else_clause,
                    elseif_clauses,
                },
            )
            .labelled("IfStatement")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        expressions::{
            primary_expression::PrimaryExpression, reserved_word_expression::ReservedWordExpression,
        },
        statements::compound_statement::CompoundStatement,
        tokenize,
    };

    use super::*;

    fn parse_elseif_clause(src: &str) -> Result<ElseIfClause, ()> {
        let tokens = tokenize(src);

        ElseIfClause::parser(Statement::parser().boxed())
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn elseif_clause_simple() {
        let res = parse_elseif_clause(r#"elseif (true) {}"#);
        assert_eq!(
            res,
            Ok(ElseIfClause {
                expression: Expression::Primary(PrimaryExpression::ReservedWord(
                    ReservedWordExpression::True
                )),
                statement: Statement::Compound(CompoundStatement { statements: vec![] })
            })
        );
    }

    fn parse_else_clause(src: &str) -> Result<ElseClause, ()> {
        let tokens = tokenize(src);

        ElseClause::parser(Statement::parser().boxed())
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn else_clause_simple() {
        let res = parse_else_clause(r#"else {}"#);
        assert_eq!(
            res,
            Ok(ElseClause(Statement::Compound(CompoundStatement {
                statements: vec![]
            })))
        );
    }

    fn parse(src: &str) -> Result<IfStatement, ()> {
        let tokens = tokenize(src);

        IfStatement::parser(Statement::parser().boxed())
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"if (true) {} elseif (false) {} else {}"#);
        assert_eq!(
            res,
            Ok(IfStatement {
                expression: Expression::Primary(PrimaryExpression::ReservedWord(
                    ReservedWordExpression::True
                )),
                statement: Statement::Compound(CompoundStatement { statements: vec![] }),
                elseif_clauses: vec![ElseIfClause {
                    expression: Expression::Primary(PrimaryExpression::ReservedWord(
                        ReservedWordExpression::False
                    )),
                    statement: Statement::Compound(CompoundStatement { statements: vec![] })
                }],
                else_clause: Some(ElseClause(Statement::Compound(CompoundStatement {
                    statements: vec![]
                })))
            })
        );
    }
}
