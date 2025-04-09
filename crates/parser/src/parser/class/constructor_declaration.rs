use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::method_modifier::MethodModifier;
use crate::parser::atoms::parameter::parameter_declaration::ParameterDeclaration;
use crate::parser::atoms::parameter::variadic_parameter::VariadicParameter;
use crate::parser::statements::compound_statement::CompoundStatement;
use crate::parser::statements::function_definition::FunctionDefinition;
use crate::parser::statements::Statement;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstructorDeclaration<'a> {
    modifiers: Vec<MethodModifier>,
    reference: bool,
    parameters: Vec<ParameterDeclaration<'a>>,
    variadic: Option<VariadicParameter<'a>>,
    body: Option<CompoundStatement<'a>>,
}

impl<'a> ConstructorDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> BoxedParser<'a, I, Self>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let modifiers = MethodModifier::list_parser();
        let reference = just(Token::Ampersand).or_not().map(|n| n.is_some());

        let header = modifiers
            .then_ignore(just(Token::FunctionKeyword))
            .then(reference)
            .then_ignore(just(Token::ConstructKeyword))
            .then(
                just(Token::OpenParen)
                    .ignore_then(FunctionDefinition::parameters_parser(
                        statement_parser.clone(),
                    ))
                    .then_ignore(just(Token::CloseParen)),
            );

        let body = choice((
            just(Token::Semicolon).map(|_| None),
            CompoundStatement::parser(statement_parser).map(Some),
        ));

        header
            .then(body)
            .map(
                |(((modifiers, reference), (parameters, variadic)), body)| Self {
                    modifiers,
                    reference,
                    parameters,
                    variadic,
                    body,
                },
            )
            .labelled("Constructor")
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{atoms::visibility_modifier::VisibilityModifier, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<ConstructorDeclaration, ()> {
        let token_stream = tokenize(src);

        ConstructorDeclaration::parser(Statement::parser())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn modifiers() {
        let res = parse(r#"public function __construct() {}"#);

        assert!(matches!(
            res.clone().unwrap().modifiers[0],
            MethodModifier::Visibility(VisibilityModifier::Public)
        ));

        assert!(matches!(
            res,
            Ok(ConstructorDeclaration {
                modifiers: _,
                reference: _,
                parameters: _,
                variadic: _,
                body: _
            })
        ));
    }
}
