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
            Token::SelfKeyword => Self::Self_,
            Token::ParentKeyworkd => Self::Parent,
            Token::StaticKeyword => Self::Static,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<RelativeScope, ()> {
        let token_stream = tokenize(src);

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
