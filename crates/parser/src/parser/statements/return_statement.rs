use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReturnStatement<'a>(pub Option<Expression<'a>>);

impl<'a> ReturnStatement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::ReturnKeyword)
            .ignore_then(Expression::parser(statement_parser).or_not())
            .then_ignore(just(Token::Semicolon))
            .map(ReturnStatement)
            .labelled("ReturnStatement")
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

    fn parse(src: &str) -> Result<ReturnStatement, ()> {
        let token_stream = tokenize(src);

        ReturnStatement::parser(Statement::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"return;"#);
        assert_eq!(res, Ok(ReturnStatement(None)));
    }

    #[test]
    fn breakout() {
        let res = parse(r#"return 5;"#);
        assert_eq!(
            res,
            Ok(ReturnStatement(Some(Expression::Primary(
                PrimaryExpression::Literal(LiteralExpression(Literal::Integer(IntegerLiteral(
                    "5"
                ))))
            ))))
        );
    }
}
