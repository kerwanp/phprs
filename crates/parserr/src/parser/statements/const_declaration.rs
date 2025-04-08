use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::const_element::ConstElement;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstDeclaration<'a>(Vec<ConstElement<'a>>);

impl<'a> ConstDeclaration<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::ConstKeyword)
            .ignore_then(ConstElement::list_parser())
            .then_ignore(just(Token::Semicolon))
            .map(Self)
            .labelled("ConstDeclaration")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        atoms::name::Name,
        expressions::{
            primary_expression::PrimaryExpression,
            reserved_word_expression::ReservedWordExpression, Expression,
        },
    };

    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<ConstDeclaration, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        ConstDeclaration::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"const HELLO = true;"#);
        assert_eq!(
            res,
            Ok(ConstDeclaration(vec![ConstElement {
                name: Name("HELLO"),
                expression: Expression::Primary(PrimaryExpression::ReservedWord(
                    ReservedWordExpression::True
                ))
            }]))
        );
    }

    #[test]
    fn list() {
        let res = parse(r#"const HELLO = true, WORLD = false;"#);
        assert_eq!(
            res,
            Ok(ConstDeclaration(vec![
                ConstElement {
                    name: Name("HELLO"),
                    expression: Expression::Primary(PrimaryExpression::ReservedWord(
                        ReservedWordExpression::True
                    ))
                },
                ConstElement {
                    name: Name("WORLD"),
                    expression: Expression::Primary(PrimaryExpression::ReservedWord(
                        ReservedWordExpression::False
                    ))
                }
            ]))
        );
    }
}
