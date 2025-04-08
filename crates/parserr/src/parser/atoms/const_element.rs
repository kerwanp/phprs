use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::expressions::Expression;

use super::name::Name;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstElement<'a> {
    pub name: Name<'a>,
    pub expression: Expression<'a>,
}

impl<'a> ConstElement<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Name::parser()
            .then_ignore(just(Token::Equals))
            .then(Expression::parser())
            .map(|(name, expression)| ConstElement { name, expression })
            .labelled("ConstElement")
    }

    pub fn list_parser<I>() -> impl Parser<'a, I, Vec<Self>, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Self::parser().separated_by(just(Token::Comma)).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::expressions::{
        primary_expression::PrimaryExpression, reserved_word_expression::ReservedWordExpression,
    };

    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<ConstElement, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        ConstElement::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    fn parse_list(src: &str) -> Result<Vec<ConstElement>, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        ConstElement::list_parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        // let res = parse(r#"HELLO = true"#);
        // assert!(matches!(
        //     res,
        //     Ok(ConstElement {
        //         name: Name("HELLO"),
        //         expression: Expression::Primary(_)
        //     })
        // ));
    }

    #[test]
    fn list() {
        let res = parse_list(r#"HELLO = true, WORLD = false"#);
        assert_eq!(
            res,
            Ok(vec![
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
            ])
        );
    }
}
