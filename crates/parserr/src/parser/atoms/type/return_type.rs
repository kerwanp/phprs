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
    use crate::parser::atoms::r#type::Type;

    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<ReturnType, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

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
