use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use super::Type;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeDeclaration<'a> {
    pub optional: bool,
    pub r#type: Type<'a>,
}

impl<'a> TypeDeclaration<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let optional = just(Token::Question).or_not().map(|t| t.is_some());

        optional
            .then(Type::parser())
            .map(|(optional, r#type)| TypeDeclaration { r#type, optional })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<TypeDeclaration, ()> {
        let token_stream = tokenize(src);

        TypeDeclaration::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"array"#);
        assert_eq!(
            res,
            Ok(TypeDeclaration {
                optional: false,
                r#type: Type::Array
            })
        );
    }

    #[test]
    fn optional() {
        let res = parse(r#"?callable"#);
        assert_eq!(
            res,
            Ok(TypeDeclaration {
                optional: true,
                r#type: Type::Callable
            })
        );
    }
}
