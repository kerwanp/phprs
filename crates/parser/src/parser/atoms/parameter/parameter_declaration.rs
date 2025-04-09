use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::atoms::name::variable_name::VariableName;
use crate::parser::atoms::r#type::type_declaration::TypeDeclaration;
use crate::parser::atoms::visibility_modifier::VisibilityModifier;
use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParameterDeclaration<'a> {
    r#type: Option<TypeDeclaration<'a>>,
    reference: bool,
    name: VariableName<'a>,
    default: Option<Expression<'a>>,
    visibility: Option<VisibilityModifier>,
}

impl<'a> ParameterDeclaration<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let visibility = VisibilityModifier::parser().or_not();
        let type_declaration = TypeDeclaration::parser().or_not();
        let reference = just(Token::Ampersand).or_not().map(|t| t.is_some());
        let variable_name = VariableName::parser();
        let default = just(Token::Equals).ignore_then(expression_parser).or_not();

        visibility
            .then(type_declaration)
            .then(reference)
            .then(variable_name)
            .then(default)
            .map(
                |((((visibility, r#type), reference), name), default)| ParameterDeclaration {
                    visibility,
                    r#type,
                    reference,
                    name,
                    default,
                },
            )
    }

    pub fn list_parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Vec<Self>, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Self::parser(expression_parser)
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        atoms::r#type::Type,
        expressions::{
            primary_expression::PrimaryExpression, reserved_word_expression::ReservedWordExpression,
        },
        statements::Statement,
        tokenize,
    };

    use super::*;

    fn parse(src: &str) -> Result<ParameterDeclaration, ()> {
        let token_stream = tokenize(src);

        ParameterDeclaration::parser(Expression::parser(Statement::parser()))
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn visibility() {
        let res = parse(r#"public $test"#);
        assert_eq!(
            res,
            Ok(ParameterDeclaration {
                visibility: Some(VisibilityModifier::Public),
                r#type: None,
                reference: false,
                name: VariableName("$test"),
                default: None,
            })
        );
    }

    #[test]
    fn simple() {
        let res = parse(r#"$test"#);
        assert_eq!(
            res,
            Ok(ParameterDeclaration {
                visibility: None,
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
                visibility: None,
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
                visibility: None,
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
                visibility: None,
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
