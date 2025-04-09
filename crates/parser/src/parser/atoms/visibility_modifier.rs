use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VisibilityModifier {
    Public,
    Protected,
    Private,
}

impl<'a> VisibilityModifier {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        select! {
            Token::PublicKeyword => Self::Public,
            Token::ProtectedKeyword => Self::Protected,
            Token::PrivateKeyword => Self::Private,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<VisibilityModifier, ()> {
        let tokens = tokenize(src);

        VisibilityModifier::parser()
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn public() {
        let res = parse(r#"public"#);
        assert_eq!(res, Ok(VisibilityModifier::Public));
    }

    #[test]
    fn protected() {
        let res = parse(r#"protected"#);
        assert_eq!(res, Ok(VisibilityModifier::Protected));
    }

    #[test]
    fn private() {
        let res = parse(r#"private"#);
        assert_eq!(res, Ok(VisibilityModifier::Private));
    }
}
