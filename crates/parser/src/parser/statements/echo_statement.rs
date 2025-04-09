use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use phprs_lexer::Token;

use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EchoStatement<'a> {
    pub expressions: Vec<Expression<'a>>,
}

impl<'a> EchoStatement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::EchoKeyword)
            .ignore_then(Expression::list_parser(statement_parser))
            .then_ignore(just(Token::Semicolon))
            .map(|expression| EchoStatement {
                expressions: expression,
            })
            .labelled("EchoStatement")
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

    fn parse(src: &str) -> Result<EchoStatement, ()> {
        let token_stream = tokenize(src);

        EchoStatement::parser(Statement::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn empty_fail() {
        let res = parse(r#"echo;"#);
        assert_eq!(res, Err(()));
    }

    #[test]
    fn simple() {
        let res = parse(r#"echo 5;"#);
        assert_eq!(
            res,
            Ok(EchoStatement {
                expressions: vec![Expression::Primary(PrimaryExpression::Literal(
                    LiteralExpression(Literal::Integer(IntegerLiteral("5")))
                ))]
            })
        );
    }
}
