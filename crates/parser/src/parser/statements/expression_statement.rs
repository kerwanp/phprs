use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExpressionStatement<'a> {
    pub expression: Option<Expression<'a>>,
}

impl<'a> ExpressionStatement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Expression::parser(statement_parser)
            .or_not()
            .then_ignore(just(Token::Semicolon))
            .map(|expression| ExpressionStatement { expression })
            .labelled("ExpressionStatement")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        atoms::literal::{integer_literal::IntegerLiteral, Literal},
        expressions::{
            literal_expression::LiteralExpression, primary_expression::PrimaryExpression,
        },
        tokenize,
    };

    use super::*;

    fn parse(src: &str) -> Result<ExpressionStatement, ()> {
        let token_stream = tokenize(src);

        ExpressionStatement::parser(Statement::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn empty() {
        let res = parse(r#";"#);
        assert_eq!(res, Ok(ExpressionStatement { expression: None }));
    }

    #[test]
    fn simple() {
        let res = parse(r#"5;"#);
        assert_eq!(
            res,
            Ok(ExpressionStatement {
                expression: Some(Expression::Primary(PrimaryExpression::Literal(
                    LiteralExpression(Literal::Integer(IntegerLiteral("5")))
                )))
            })
        );
    }
}
