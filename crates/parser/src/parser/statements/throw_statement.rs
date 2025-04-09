use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::expressions::Expression;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ThrowStatement<'a>(pub Option<Expression<'a>>);

impl<'a> ThrowStatement<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::ThrowKeyword)
            .ignore_then(Expression::parser().or_not())
            .then_ignore(just(Token::Semicolon))
            .map(ThrowStatement)
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

    fn parse(src: &str) -> Result<ThrowStatement, ()> {
        let token_stream = tokenize(src);

        ThrowStatement::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"throw;"#);
        assert_eq!(res, Ok(ThrowStatement(None)));
    }

    #[test]
    fn breakout() {
        let res = parse(r#"throw 5;"#);
        assert_eq!(
            res,
            Ok(ThrowStatement(Some(Expression::Primary(
                PrimaryExpression::Literal(LiteralExpression(Literal::Integer(IntegerLiteral(
                    "5"
                ))))
            ))))
        );
    }
}
