use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CaseStatement<'a> {
    // Default case if None
    expression: Option<Expression<'a>>,
    terminator: Token<'a>, // TODO: Could be an enum for ; or :
    statements: Vec<Statement<'a>>,
}

impl<'a> CaseStatement<'a> {
    pub fn parser<I>(
        statements_parser: BoxedParser<'a, I, Vec<Statement<'a>>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let terminator = choice((just(Token::Colon), just(Token::Semicolon)));

        let head = just(Token::CaseKeyword)
            .ignore_then(Expression::parser())
            .map(Some);
        let default_head = just(Token::DefaultKeyword).map(|_| None);

        head.or(default_head)
            .then(terminator)
            .then(statements_parser)
            .map(|((expression, terminator), statements)| CaseStatement {
                expression,
                terminator,
                statements,
            })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SwitchStatement<'a> {
    expression: Expression<'a>,
    cases: Vec<CaseStatement<'a>>,
}

impl<'a> SwitchStatement<'a> {
    pub fn parser<I>(
        statements_parser: BoxedParser<'a, I, Vec<Statement<'a>>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let expr =
            Expression::parser().delimited_by(just(Token::OpenParen), just(Token::CloseParen));

        let switch = just(Token::SwitchKeyword).ignore_then(expr);
        let body1 = just(Token::OpenBrace)
            .ignore_then(
                CaseStatement::parser(statements_parser.clone())
                    .repeated()
                    .collect(),
            )
            .then_ignore(just(Token::CloseBrace));
        let body2 = just(Token::Colon)
            .ignore_then(
                CaseStatement::parser(statements_parser)
                    .repeated()
                    .collect(),
            )
            .then_ignore(just(Token::EndSwitchKeyword));

        switch
            .then(body1.or(body2))
            .map(|(expression, cases)| SwitchStatement { expression, cases })
            .labelled("SwitchStatement")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        atoms::literal::{integer_literal::IntegerLiteral, Literal},
        expressions::{
            literal_expression::LiteralExpression, primary_expression::PrimaryExpression,
        },
        statements::expression_statement::ExpressionStatement,
    };

    use super::*;
    use chumsky::input::Stream;

    fn parse_case_statement(src: &str) -> Result<CaseStatement, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        CaseStatement::parser(Statement::list_parser(Statement::parser().boxed()).boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn case_simple() {
        let res = parse_case_statement(r#"case 5;"#);
        assert_eq!(
            res,
            Ok(CaseStatement {
                expression: Some(Expression::Primary(PrimaryExpression::Literal(
                    LiteralExpression(Literal::Integer(IntegerLiteral("5")))
                ))),
                terminator: Token::Semicolon,
                statements: vec![]
            })
        );
    }

    #[test]
    fn case_complex() {
        let res = parse_case_statement(r#"case 5:; "#);
        assert_eq!(
            res,
            Ok(CaseStatement {
                expression: Some(Expression::Primary(PrimaryExpression::Literal(
                    LiteralExpression(Literal::Integer(IntegerLiteral("5")))
                ))),
                terminator: Token::Colon,
                statements: vec![Statement::Expression(ExpressionStatement {
                    expression: None
                })]
            })
        );
    }

    #[test]
    fn default_case() {
        let res = parse_case_statement(r#"default:;"#);
        assert_eq!(
            res,
            Ok(CaseStatement {
                expression: None,
                terminator: Token::Colon,
                statements: vec![Statement::Expression(ExpressionStatement {
                    expression: None
                })]
            })
        );
    }

    fn parse(src: &str) -> Result<SwitchStatement, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        SwitchStatement::parser(Statement::list_parser(Statement::parser().boxed()).boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn empty() {
        let res = parse(r#"switch(5){}"#);
        assert_eq!(
            res,
            Ok(SwitchStatement {
                expression: Expression::Primary(PrimaryExpression::Literal(LiteralExpression(
                    Literal::Integer(IntegerLiteral("5"))
                ))),
                cases: vec![]
            })
        );
    }

    #[test]
    fn simple() {
        let res = parse(
            r#"
        switch(5) {
            case 5:
            default:;
        }
        "#,
        );
        assert_eq!(
            res,
            Ok(SwitchStatement {
                expression: Expression::Primary(PrimaryExpression::Literal(LiteralExpression(
                    Literal::Integer(IntegerLiteral("5"))
                ))),
                cases: vec![
                    CaseStatement {
                        expression: Some(Expression::Primary(PrimaryExpression::Literal(
                            LiteralExpression(Literal::Integer(IntegerLiteral("5")))
                        ))),
                        terminator: Token::Colon,
                        statements: vec![]
                    },
                    CaseStatement {
                        expression: None,
                        terminator: Token::Colon,
                        statements: vec![Statement::Expression(ExpressionStatement {
                            expression: None
                        })]
                    }
                ]
            })
        );
    }
}
