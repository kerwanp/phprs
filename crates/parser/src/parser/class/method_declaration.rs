use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::method_modifier::MethodModifier;
use crate::parser::atoms::name::Name;
use crate::parser::atoms::parameter::parameter_declaration::ParameterDeclaration;
use crate::parser::atoms::parameter::variadic_parameter::VariadicParameter;
use crate::parser::atoms::r#type::return_type::ReturnType;
use crate::parser::statements::compound_statement::CompoundStatement;
use crate::parser::statements::function_definition::FunctionDefinition;
use crate::parser::statements::Statement;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MethodDeclaration<'a> {
    name: Name<'a>,
    modifiers: Vec<MethodModifier>,
    reference: bool,
    parameters: Vec<ParameterDeclaration<'a>>,
    variadic: Option<VariadicParameter<'a>>,
    return_type: Option<ReturnType<'a>>,
    body: Option<CompoundStatement<'a>>,
}

impl<'a> MethodDeclaration<'a> {
    // TODO: Should directly use FunctionDefinition::parser() once header is splitted
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
            .then(Name::parser())
            .then(
                just(Token::OpenParen)
                    .ignore_then(FunctionDefinition::parameters_parser(
                        statement_parser.clone(),
                    ))
                    .then_ignore(just(Token::CloseParen)),
            )
            .then(ReturnType::parser().or_not());

        let body = choice((
            just(Token::Semicolon).map(|_| None),
            CompoundStatement::parser(statement_parser).map(Some),
        ));

        header
            .then(body)
            .map(
                |(
                    ((((modifiers, reference), name), (parameters, variadic)), return_type),
                    body,
                )| {
                    Self {
                        name,
                        modifiers,
                        reference,
                        parameters,
                        variadic,
                        return_type,
                        body,
                    }
                },
            )
            .boxed()
    }
}
