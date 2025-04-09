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
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<VariableName, ()> {
        let token_stream = tokenize(src);

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
