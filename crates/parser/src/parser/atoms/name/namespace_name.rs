use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamespaceName<'a>(pub Vec<&'a str>);

impl<'a> NamespaceName<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let name = select! {
            Token::Name(name) => name
        };

        name.separated_by(just(Token::Backslack))
            .at_least(1)
            .collect::<Vec<_>>()
            .map(NamespaceName)
            .labelled("Namespace name")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<NamespaceName, ()> {
        let token_stream = tokenize(src);

        NamespaceName::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"App"#);
        assert_eq!(res, Ok(NamespaceName(vec!["App"])));
    }

    #[test]
    fn multiple() {
        let res = parse(r#"App\Test"#);
        assert_eq!(res, Ok(NamespaceName(vec!["App", "Test"])));
    }

    #[test]
    fn long() {
        let res = parse(r#"App\test_\HELLoo\W\O\R\L\D"#);
        assert_eq!(
            res,
            Ok(NamespaceName(vec![
                "App", "test_", "HELLoo", "W", "O", "R", "L", "D"
            ]))
        );
    }
}
