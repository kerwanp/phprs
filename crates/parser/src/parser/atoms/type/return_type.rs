use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use super::type_declaration::TypeDeclaration;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ReturnType {
    Void,
    Declaration(TypeDeclaration),
}

impl<'a> ReturnType {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let void = just(Token::VoidReservedWord).map(|_| ReturnType::Void);
        let declaration = TypeDeclaration::parser().map(ReturnType::Declaration);

        just(Token::Colon).ignore_then(void.or(declaration))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{atoms::r#type::Type, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<ReturnType, ()> {
        let token_stream = tokenize(src);

        ReturnType::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn void() {
        let res = parse(r#": void"#);
        assert_eq!(res, Ok(ReturnType::Void));
    }

    #[test]
    fn optional_type_declaration() {
        let res = parse(r#": ?array"#);
        assert_eq!(
            res,
            Ok(ReturnType::Declaration(TypeDeclaration {
                optional: true,
                r#type: Type::Array
            }))
        );
    }
}
