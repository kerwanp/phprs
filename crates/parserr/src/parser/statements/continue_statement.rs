use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::breakout_level::BreakoutLevel;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContinueStatement<'a>(pub Option<BreakoutLevel<'a>>);

impl<'a> ContinueStatement<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::ContinueKeyword)
            .ignore_then(BreakoutLevel::parser().or_not())
            .then_ignore(just(Token::Semicolon))
            .map(ContinueStatement)
            .labelled("ContinueStatement")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<ContinueStatement, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        ContinueStatement::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"continue;"#);
        assert_eq!(res, Ok(ContinueStatement(None)));
    }

    #[test]
    fn breakout() {
        // let res = parse(r#"continue 5;"#);
        // assert!(matches!(
        //     res,
        //     Ok(ContinueStatement(Some(BreakoutLevel::Level(_))))
        // ));
    }
}
