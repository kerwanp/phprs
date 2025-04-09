use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::atoms::name::Name;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamedLabelStatement<'a>(Name<'a>);

impl<'a> NamedLabelStatement<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Name::parser()
            .then_ignore(just(Token::Colon))
            .map(NamedLabelStatement)
            .labelled("NamedLabelStatement")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<NamedLabelStatement, ()> {
        let tokens = tokenize(src);

        NamedLabelStatement::parser()
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"test:"#);
        assert_eq!(res, Ok(NamedLabelStatement(Name("test"))));
    }

    #[test]
    fn no_colon_fail() {
        let res = parse(r#"test"#);
        assert_eq!(res, Err(()));
    }

    #[test]
    fn no_label_fail() {
        let res = parse(r#":"#);
        assert_eq!(res, Err(()));
    }
}
