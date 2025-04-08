use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhileStatement<'a> {
    expression: Expression<'a>,
    statements: Vec<Statement<'a>>,
}

impl<'a> WhileStatement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let head = just(Token::WhileKeyword).ignore_then(
            Expression::parser().delimited_by(just(Token::OpenParen), just(Token::CloseParen)),
        );

        let body1 = statement_parser.clone().map(|s| vec![s]);
        let body2 = just(Token::Colon)
            .ignore_then(Statement::list_parser(statement_parser))
            .then_ignore(just(Token::EndWhileKeyword))
            .then_ignore(just(Token::Semicolon));

        head.then(body1.or(body2))
            .map(|(expression, statements)| WhileStatement {
                expression,
                statements,
            })
            .labelled("WhileStatement")
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

    fn parse(src: &str) -> Result<WhileStatement, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        WhileStatement::parser(Statement::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"while(5);"#);
        assert_eq!(
            res,
            Ok(WhileStatement {
                expression: Expression::Primary(PrimaryExpression::Literal(LiteralExpression(
                    Literal::Integer(IntegerLiteral("5"))
                ))),
                statements: vec![Statement::Expression(ExpressionStatement {
                    expression: None
                })]
            })
        );
    }

    #[test]
    fn keywords() {
        let res = parse(r#"while(5):;endwhile;"#);
        assert_eq!(
            res,
            Ok(WhileStatement {
                expression: Expression::Primary(PrimaryExpression::Literal(LiteralExpression(
                    Literal::Integer(IntegerLiteral("5"))
                ))),
                statements: vec![Statement::Expression(ExpressionStatement {
                    expression: None
                })]
            })
        );
    }
}
