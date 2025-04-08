use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::literal::string_literal::StringLiteral;
use crate::parser::variables::callable::CallableVariable;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::array_creation_expression::ArrayCreationExpression;
use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CallableExpression<'a> {
    CallableVariable(CallableVariable<'a>),
    Expression(Expression<'a>),
    ArrayCreation(ArrayCreationExpression<'a>),
    StringLiteral(StringLiteral<'a>),
}

impl<'a> CallableExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let callable =
            CallableVariable::parser(expression_parser.clone()).map(Self::CallableVariable);

        let expression = just(Token::OpenParen)
            .ignore_then(expression_parser.clone())
            .then_ignore(just(Token::CloseParen))
            .map(Self::Expression);

        let array_creation =
            ArrayCreationExpression::parser(expression_parser).map(Self::ArrayCreation);

        let string_literal = StringLiteral::parser().map(Self::StringLiteral);

        choice((callable, expression, array_creation, string_literal))
    }
}
