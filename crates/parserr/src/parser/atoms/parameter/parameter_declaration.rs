use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::atoms::name::variable_name::VariableName;
use crate::parser::atoms::r#type::type_declaration::TypeDeclaration;
use crate::parser::expressions::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParameterDeclaration<'a> {
    r#type: Option<TypeDeclaration>,
    reference: bool,
    name: VariableName<'a>,
    default: Option<Expression<'a>>,
}

impl<'a> ParameterDeclaration<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let type_declaration = TypeDeclaration::parser().or_not();
        let reference = just(Token::Ampersand).or_not().map(|t| t.is_some());
        let variable_name = VariableName::parser();
        let default = just(Token::Equals)
            .ignore_then(Expression::parser())
            .or_not();

        type_declaration
            .then(reference)
            .then(variable_name)
            .then(default)
            .map(
                |(((r#type, reference), name), default)| ParameterDeclaration {
                    r#type,
                    reference,
                    name,
                    default,
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        atoms::r#type::Type,
        expressions::{
            primary_expression::PrimaryExpression, reserved_word_expression::ReservedWordExpression,
        },
    };

    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<ParameterDeclaration, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        ParameterDeclaration::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"$test"#);
        assert_eq!(
            res,
            Ok(ParameterDeclaration {
                r#type: None,
                reference: false,
                name: VariableName("$test"),
                default: None,
            })
        );
    }

    #[test]
    fn reference() {
        let res = parse(r#"&$test"#);
        assert_eq!(
            res,
            Ok(ParameterDeclaration {
                r#type: None,
                reference: true,
                name: VariableName("$test"),
                default: None,
            })
        );
    }

    #[test]
    fn typed() {
        let res = parse(r#"array $test"#);
        assert_eq!(
            res,
            Ok(ParameterDeclaration {
                r#type: Some(TypeDeclaration {
                    optional: false,
                    r#type: Type::Array
                }),
                reference: false,
                name: VariableName("$test"),
                default: None,
            })
        );
    }

    #[test]
    fn default() {
        let res = parse(r#"$test = true"#);
        assert_eq!(
            res,
            Ok(ParameterDeclaration {
                r#type: None,
                reference: false,
                name: VariableName("$test"),
                default: Some(Expression::Primary(PrimaryExpression::ReservedWord(
                    ReservedWordExpression::True
                ))),
            })
        );
    }
}
