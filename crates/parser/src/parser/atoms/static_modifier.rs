use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StaticModifier;

impl<'a> StaticModifier {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        select! {
            Token::StaticKeyword => Self,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<StaticModifier, ()> {
        let token_stream = tokenize(src);

        StaticModifier::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"static"#);
        assert_eq!(res, Ok(StaticModifier));
    }
}
