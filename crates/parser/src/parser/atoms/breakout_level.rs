use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use super::literal::integer_literal::IntegerLiteral;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BreakoutLevel<'a> {
    Level(IntegerLiteral<'a>),
    Breakout(Box<Self>),
}

impl<'a> BreakoutLevel<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        recursive(|parser| {
            let literal = IntegerLiteral::parser().map(BreakoutLevel::Level);
            let breakout = just(Token::OpenParen)
                .ignore_then(parser)
                .then_ignore(just(Token::CloseParen))
                .map(|breakout| Self::Breakout(Box::new(breakout)));

            literal.or(breakout)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<BreakoutLevel, ()> {
        let token_stream = tokenize(src);

        BreakoutLevel::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"5"#);
        assert!(matches!(res, Ok(BreakoutLevel::Level(_))));
    }

    #[test]
    fn nested() {
        let res = parse(r#"(5)"#);
        assert!(matches!(res, Ok(BreakoutLevel::Breakout(_))));
    }
}
