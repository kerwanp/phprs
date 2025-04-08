use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::expressions::Expression;
use crate::parser::variables::Variable;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ListOrVariable<'a> {
    Variable {
        reference: bool,
        variable: Variable<'a>,
    },
    List(ListIntrinsic<'a>),
}

impl<'a> ListOrVariable<'a> {
    pub fn parser<I>(
        list_instrinsic: BoxedParser<'a, I, ListIntrinsic<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let variable = just(Token::Ampersand)
            .or_not()
            .map(|t| t.is_some())
            .then(Variable::parser(Expression::parser().boxed()))
            .map(|(reference, variable)| Self::Variable {
                reference,
                variable,
            });

        let list_instrinsic = list_instrinsic.map(Self::List);

        variable.or(list_instrinsic)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ListIntrinsic<'a> {
    Keyed(Vec<(Expression<'a>, ListOrVariable<'a>)>),
    Unkeyed(Vec<ListOrVariable<'a>>),
}

impl<'a> ListIntrinsic<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        recursive(|list_intrinsic| {
            let keyed = Expression::parser()
                .then_ignore(just(Token::DoubleArrow))
                .then(ListOrVariable::parser(list_intrinsic.clone().boxed()))
                .map(|(expression, value)| (expression, value))
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .collect()
                .map(Self::Keyed);

            let unkeyed = ListOrVariable::parser(list_intrinsic.boxed())
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .collect()
                .map(Self::Unkeyed);

            just(Token::ListKeyword)
                .ignore_then(just(Token::OpenParen))
                .ignore_then(keyed.or(unkeyed))
                .then_ignore(just(Token::CloseParen))
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ForeachValue<'a> {
    Expression {
        reference: bool,
        expression: Expression<'a>,
    },
    ListIntrinsic(ListIntrinsic<'a>),
}

impl<'a> ForeachValue<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let expression = just(Token::Ampersand)
            .or_not()
            .map(|t| t.is_some())
            .then(Expression::parser())
            .map(|(reference, expression)| Self::Expression {
                reference,
                expression,
            });

        let list_intrinsic = ListIntrinsic::parser().map(Self::ListIntrinsic);

        expression.or(list_intrinsic)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForeachStatement<'a> {
    pub collection_name: Expression<'a>,
    pub key: Option<Expression<'a>>,
    pub value: ForeachValue<'a>,
    pub body: Vec<Statement<'a>>,
}

impl<'a> ForeachStatement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let foreach_key = Expression::parser()
            .then_ignore(just(Token::DoubleArrow))
            .or_not();

        let header = just(Token::ForeachKeyword)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(Expression::parser())
            .then_ignore(just(Token::AsKeyword))
            .then(foreach_key)
            .then(ForeachValue::parser())
            .then_ignore(just(Token::CloseParen))
            .map(|((collection_name, key), value)| (collection_name, key, value));

        let body = statement_parser
            .clone()
            .map(|s| vec![s])
            .or(just(Token::Colon)
                .ignore_then(Statement::list_parser(statement_parser))
                .then_ignore(just(Token::EndForEachKeyword))
                .then_ignore(just(Token::Semicolon)));

        header
            .then(body)
            .map(|((collection_name, key, value), body)| Self {
                collection_name,
                key,
                value,
                body,
            })
    }
}
