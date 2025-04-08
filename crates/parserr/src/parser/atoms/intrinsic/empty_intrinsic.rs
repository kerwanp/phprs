use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EmptyIntrinsic<'a>(Expression<'a>);

impl<'a> EmptyIntrinsic<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::EmptyKeyword)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(expression_parser)
            .then_ignore(just(Token::CloseParen))
            .map(Self)
            .labelled("EmptyIntrinsic")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<EmptyIntrinsic, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        EmptyIntrinsic::parser(Expression::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        // let res = parse(r#"empty(5)"#);
        // assert!(matches!(res, Ok(EmptyIntrinsic(Expression::Primary(_)))));
    }
}
