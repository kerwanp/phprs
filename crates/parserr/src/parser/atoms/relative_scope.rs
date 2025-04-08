use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RelativeScope {
    Self_,
    Parent,
    Static,
}

impl<'a> RelativeScope {
    // TODO: Could be tokens
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        select! {
            Token::Name("self") => Self::Self_,
            Token::Name("parent") => Self::Parent,
            Token::StaticKeyword => Self::Static,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<RelativeScope, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        RelativeScope::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn self_() {
        let res = parse(r#"self"#);
        assert_eq!(res, Ok(RelativeScope::Self_));
    }

    #[test]
    fn parent() {
        let res = parse(r#"parent"#);
        assert_eq!(res, Ok(RelativeScope::Parent));
    }

    #[test]
    fn static_() {
        let res = parse(r#"static"#);
        assert_eq!(res, Ok(RelativeScope::Static));
    }
}
