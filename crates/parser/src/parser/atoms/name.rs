pub mod namespace_name;
pub mod qualified_name;
pub mod variable_name;

use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name<'a>(pub &'a str);

impl<'a> Name<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        select! {
            Token::Name(name) => name
        }
        .map(Name)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<Name, ()> {
        let token_stream = tokenize(src);

        Name::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"test"#);
        assert_eq!(res, Ok(Name("test")));
    }

    #[test]
    fn complex() {
        let res = parse(r#"hello_WoRLD"#);
        assert_eq!(res, Ok(Name("hello_WoRLD")));
    }
}
