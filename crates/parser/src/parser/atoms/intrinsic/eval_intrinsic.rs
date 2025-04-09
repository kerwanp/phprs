use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EvalIntrinsic<'a>(Expression<'a>);

impl<'a> EvalIntrinsic<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::EvalKeyword)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(expression_parser)
            .then_ignore(just(Token::CloseParen))
            .map(Self)
            .labelled("EvalIntrinsic")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{statements::Statement, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<EvalIntrinsic, ()> {
        let token_stream = tokenize(src);

        EvalIntrinsic::parser(Expression::parser(Statement::parser()))
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"eval(5)"#);
        assert!(matches!(res, Ok(EvalIntrinsic(Expression::Primary(_)))));
    }
}
