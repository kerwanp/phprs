use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::expressions::Expression;
use crate::parser::statements::Statement;
use crate::parser::BoxedParser;

use super::name::Name;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstElement<'a> {
    pub name: Name<'a>,
    pub expression: Expression<'a>,
}

impl<'a> ConstElement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Name::parser()
            .then_ignore(just(Token::Equals))
            .then(Expression::parser(statement_parser))
            .map(|(name, expression)| ConstElement { name, expression })
            .labelled("ConstElement")
    }

    pub fn list_parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Vec<Self>, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Self::parser(statement_parser)
            .separated_by(just(Token::Comma))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        expressions::{
            primary_expression::PrimaryExpression, reserved_word_expression::ReservedWordExpression,
        },
        tokenize,
    };

    use super::*;

    fn parse(src: &str) -> Result<ConstElement, ()> {
        let token_stream = tokenize(src);

        ConstElement::parser(Statement::parser())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    fn parse_list(src: &str) -> Result<Vec<ConstElement>, ()> {
        let token_stream = tokenize(src);

        ConstElement::list_parser(Statement::parser())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"HELLO = true"#);
        assert!(matches!(
            res,
            Ok(ConstElement {
                name: Name("HELLO"),
                expression: Expression::Primary(_)
            })
        ));
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
