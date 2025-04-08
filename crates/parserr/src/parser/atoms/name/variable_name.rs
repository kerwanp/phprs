use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableName<'a>(pub &'a str);

impl<'a> VariableName<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        select! {
            Token::VariableName(name) => name
        }
        .map(VariableName)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<VariableName, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        VariableName::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"$hello_WORLD"#);
        assert_eq!(res, Ok(VariableName("$hello_WORLD")));
    }
}
