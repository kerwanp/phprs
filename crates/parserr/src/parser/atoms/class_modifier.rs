use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ClassModifier {
    Abstract,
    Final,
}

impl<'a> ClassModifier {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        select! {
            Token::AbstractKeyword => Self::Abstract,
            Token::FinalKeyword => Self::Final
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<ClassModifier, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        ClassModifier::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn abstract_() {
        let res = parse(r#"abstract"#);
        assert_eq!(res, Ok(ClassModifier::Abstract));
    }

    #[test]
    fn final_() {
        let res = parse(r#"final"#);
        assert_eq!(res, Ok(ClassModifier::Final));
    }
}
