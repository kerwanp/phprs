use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use crate::parser::expressions::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExpressionStatement<'a> {
    pub expression: Option<Expression<'a>>,
}

impl<'a> ExpressionStatement<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Expression::parser()
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
    };

    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<ExpressionStatement, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        ExpressionStatement::parser()
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
