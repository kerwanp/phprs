use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExitIntrinsic<'a>(Option<Expression<'a>>);

impl<'a> ExitIntrinsic<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let body = just(Token::OpenParen)
            .ignore_then(expression_parser.or_not())
            .then_ignore(just(Token::CloseParen))
            .or_not()
            .map(|t| t.unwrap_or_default());

        choice((just(Token::ExitKeyword), just(Token::DieKeyword)))
            .ignore_then(body)
            .map(Self)
            .labelled("ExitIntrinsic")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<ExitIntrinsic, ()> {
        let token_stream = tokenize(src);

        ExitIntrinsic::parser(Expression::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn exit_parens() {
        let res = parse(r#"exit(5)"#);
        assert!(matches!(
            res,
            Ok(ExitIntrinsic(Some(Expression::Primary(_))))
        ));
    }

    #[test]
    fn die_parens() {
        let res = parse(r#"exit(5)"#);
        assert!(matches!(
            res,
            Ok(ExitIntrinsic(Some(Expression::Primary(_))))
        ));
    }

    #[test]
    fn exit() {
        let res = parse(r#"exit"#);
        assert_eq!(res, Ok(ExitIntrinsic(None)));
    }

    #[test]
    fn die() {
        let res = parse(r#"die"#);
        assert_eq!(res, Ok(ExitIntrinsic(None)));
    }
}
