use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::name::Name;
use crate::parser::atoms::parameter::parameter_declaration::ParameterDeclaration;
use crate::parser::atoms::parameter::variadic_parameter::VariadicParameter;
use crate::parser::atoms::r#type::return_type::ReturnType;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::compound_statement::CompoundStatement;
use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionDefinition<'a> {
    name: Name<'a>,
    reference: bool,
    parameters: Vec<ParameterDeclaration<'a>>,
    variadic: Option<VariadicParameter<'a>>,
    return_type: Option<ReturnType>,
    body: CompoundStatement<'a>,
}

impl<'a> FunctionDefinition<'a> {
    pub fn parameters_parser<I>() -> impl Parser<
        'a,
        I,
        (Vec<ParameterDeclaration<'a>>, Option<VariadicParameter<'a>>),
        extra::Err<Rich<'a, Token<'a>>>,
    >
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let parameters = ParameterDeclaration::parser()
            .separated_by(just(Token::Comma))
            .collect();

        let variadic = VariadicParameter::parser();

        let opt = variadic
            .clone()
            .or_not()
            .map(|v| (vec![], v))
            .labelled("VariadicParameter");

        parameters
            .then(
                just(Token::Comma)
                    .ignore_then(variadic)
                    .or_not()
                    .labelled("VariadicParameter"),
            )
            .labelled("Parameters")
            .or(opt)
    }

    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let start = just(Token::FunctionKeyword)
            .ignore_then(just(Token::Ampersand).or_not().map(|t| t.is_some()))
            .then(Name::parser());

        let parameters = just(Token::OpenParen)
            .ignore_then(Self::parameters_parser())
            .then_ignore(just(Token::CloseParen));

        start
            .then(parameters)
            .then(ReturnType::parser().or_not())
            .then(CompoundStatement::parser(statement_parser))
            .map(
                |((((reference, name), (parameters, variadic)), return_type), body)| {
                    FunctionDefinition {
                        name,
                        reference,
                        parameters,
                        variadic,
                        return_type,
                        body,
                    }
                },
            )
            .labelled("FunctionDefinition")
    }
}
