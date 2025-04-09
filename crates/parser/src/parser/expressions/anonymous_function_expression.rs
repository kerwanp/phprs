use crate::parser::atoms::parameter::parameter_declaration::ParameterDeclaration;
use crate::parser::atoms::r#type::return_type::ReturnType;
use crate::parser::statements::compound_statement::CompoundStatement;
use crate::parser::statements::Statement;
use crate::parser::BoxedParser;
use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousFunctionCreationExpression<'a> {
    pub static_: bool,
    pub reference: bool,
    pub parameters: Vec<ParameterDeclaration<'a>>,
    // TODO: Use clause
    // pub body: CompoundStatement<'a>,
    pub return_type: Option<ReturnType>,
}

impl<'a> AnonymousFunctionCreationExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let static_ = just(Token::StaticKeyword).or_not().map(|t| t.is_some());
        let reference = just(Token::Ampersand).or_not().map(|t| t.is_some());
        let parameters = just(Token::OpenParen)
            .ignore_then(ParameterDeclaration::list_parser(expression_parser))
            .then_ignore(just(Token::CloseParen));
        // TODO: Use clause
        let return_type = ReturnType::parser().or_not();

        // // TODO: might make cycles
        // let body = CompoundStatement::parser(Statement::parser().boxed());

        static_
            .then_ignore(just(Token::FunctionKeyword))
            .then(reference)
            .then(parameters)
            .then(return_type)
            // .then(body)
            .map(|((((static_, reference), parameters), return_type))| Self {
                static_,
                reference,
                parameters,
                // body,
                return_type,
            })
            .labelled("AnonymousFunctionExpression")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<AnonymousFunctionCreationExpression, ()> {
        let token_stream = tokenize(src);

        AnonymousFunctionCreationExpression::parser(Expression::parser())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        assert!(matches!(
            parse(r#"static function ()"#),
            Ok(AnonymousFunctionCreationExpression { .. })
        ));
    }
}
