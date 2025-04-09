use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::name::Name;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GotoStatement<'a>(pub Name<'a>);

impl<'a> GotoStatement<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::GotoKeyword)
            .ignore_then(Name::parser())
            .then_ignore(just(Token::Semicolon))
            .map(GotoStatement)
            .labelled("GotoStatement")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<GotoStatement, ()> {
        let token_stream = tokenize(src);

        GotoStatement::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"goto test;"#);
        assert_eq!(res, Ok(GotoStatement(Name("test"))));
    }
}
