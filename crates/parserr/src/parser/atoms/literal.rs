pub mod binary_literal;
pub mod floating_literal;
pub mod integer_literal;
pub mod string_literal;

use binary_literal::BinaryLiteral;
use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use floating_literal::FloatingLiteral;
use integer_literal::IntegerLiteral;
use phprs_lexer::Token;
use string_literal::StringLiteral;

// TODO: Separate in multiple struct
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Literal<'a> {
    String(StringLiteral<'a>),
    Integer(IntegerLiteral<'a>),
    Floating(FloatingLiteral<'a>),
    Binary(BinaryLiteral<'a>),
}

impl<'a> Literal<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        choice((
            StringLiteral::parser().map(Self::String),
            IntegerLiteral::parser().map(Self::Integer),
            FloatingLiteral::parser().map(Self::Floating),
            BinaryLiteral::parser().map(Self::Binary),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<Literal, ()> {
        let tokens = tokenize(src);

        Literal::parser()
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn string() {
        let res = parse(r#"'hello'"#);
        assert_eq!(res, Ok(Literal::String(StringLiteral("'hello'"))));
    }

    #[test]
    fn integer() {
        let res = parse(r#"12345"#);
        assert_eq!(res, Ok(Literal::Integer(IntegerLiteral("12345"))));
    }

    #[test]
    fn float() {
        let res = parse(r#"12.34"#);
        assert_eq!(res, Ok(Literal::Floating(FloatingLiteral("12.34"))));
    }

    #[test]
    fn binary() {
        let res = parse(r#"0b0100"#);
        assert_eq!(res, Ok(Literal::Binary(BinaryLiteral("0b0100"))));
    }
}
