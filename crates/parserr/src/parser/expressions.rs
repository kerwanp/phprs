use chumsky::input::ValueInput;
use chumsky::pratt::*;
use chumsky::prelude::*;
use chumsky::Parser;
use phprs_lexer::Token;
use primary_expression::PrimaryExpression;

use super::BoxedParser;

pub mod array_creation_expression;
pub mod byref_assignment_expression;
pub mod callable_expression;
pub mod class_constant_access_expression;
pub mod constant_access_expression;
pub mod decrement_expression;
pub mod include_expression;
pub mod include_once_expression;
pub mod increment_expression;
pub mod literal_expression;
pub mod primary_expression;
pub mod require_expression;
pub mod require_once_expression;
pub mod reserved_word_expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression<'a> {
    Primary(PrimaryExpression<'a>),
    Clone(Box<Self>),
    Power(Box<Self>, Box<Self>),
    Pos(Box<Self>),
    Neg(Box<Self>),
    BitwiseNot(Box<Self>),
    ErrorControl(Box<Self>),
    Cast(Box<Self>),       // TODO
    InstanceOf(Box<Self>), // TODO
    LogicalNot(Box<Self>),
    Multiply(Box<Self>, Box<Self>),
    Divide(Box<Self>, Box<Self>),
    Modulo(Box<Self>, Box<Self>),
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Concatenate(Box<Self>, Box<Self>),
    LeftShift(Box<Self>, Box<Self>),
    RightShift(Box<Self>, Box<Self>),
    LessThan(Box<Self>, Box<Self>),
    GreaterThan(Box<Self>, Box<Self>),
    LessThanOrEqual(Box<Self>, Box<Self>),
    GreaterThanOrEqual(Box<Self>, Box<Self>),
    Spaceship(Box<Self>, Box<Self>),
    Equal(Box<Self>, Box<Self>),
    NotEqual(Box<Self>, Box<Self>),
    Inequal(Box<Self>, Box<Self>), // TODO: Maybe better name?
    StrictEqual(Box<Self>, Box<Self>),
    StrictNotEqual(Box<Self>, Box<Self>),
    BitwiseAnd(Box<Self>, Box<Self>),
    BitwiseExc(Box<Self>, Box<Self>),
    BitwiseInc(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Coalesce(Box<Self>, Box<Self>),
    Conditional, // TODO
}

impl<'a> Expression<'a> {
    // pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    pub fn parser<I>() -> BoxedParser<'a, I, Self>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        recursive(|expression_parser| {
            let atom =
                PrimaryExpression::parser(expression_parser.boxed()).map(Expression::Primary);

            atom.pratt((
                // CLONE
                prefix(100, just(Token::CloneKeyword), |_, r, _| {
                    Expression::Clone(Box::new(r))
                }),
                // UNARY
                prefix(99, just(Token::Plus), |_, r, _| {
                    Expression::Pos(Box::new(r))
                }),
                prefix(99, just(Token::Minus), |_, r, _| {
                    Expression::Neg(Box::new(r))
                }),
                prefix(99, just(Token::Tilde), |_, r, _| {
                    Expression::BitwiseNot(Box::new(r))
                }),
                prefix(99, just(Token::AtSymbol), |_, r, _| {
                    Expression::ErrorControl(Box::new(r))
                }),
                // INSTANCEOF
                // TODO: prefix instanceof 98
                // LOGICAL NOT
                prefix(97, just(Token::Exclamation), |_, r, _| {
                    Expression::LogicalNot(Box::new(r))
                }),
                infix(left(99), just(Token::AsteriskAsterisk), |l, _, r, _| {
                    Expression::Power(Box::new(l), Box::new(r))
                }),
                // MULTIPLICATIVE
                infix(left(96), just(Token::Asterisk), |l, _, r, _| {
                    Expression::Multiply(Box::new(l), Box::new(r))
                }),
                infix(left(96), just(Token::Percent), |l, _, r, _| {
                    Expression::Modulo(Box::new(l), Box::new(r))
                }),
                // ADDITIVE
                infix(left(95), just(Token::Plus), |l, _, r, _| {
                    Expression::Add(Box::new(l), Box::new(r))
                }),
                infix(left(95), just(Token::Minus), |l, _, r, _| {
                    Expression::Sub(Box::new(l), Box::new(r))
                }),
                infix(left(95), just(Token::Dot), |l, _, r, _| {
                    Expression::Concatenate(Box::new(l), Box::new(r))
                }),
                // SHIFT
                infix(left(94), just(Token::LessThanLessThan), |l, _, r, _| {
                    Expression::LeftShift(Box::new(l), Box::new(r))
                }),
                infix(
                    left(94),
                    just(Token::GreaterThanGreaterThan),
                    |l, _, r, _| Expression::RightShift(Box::new(l), Box::new(r)),
                ),
                // RELATIONAL
                infix(left(93), just(Token::LessThan), |l, _, r, _| {
                    Expression::LessThan(Box::new(l), Box::new(r))
                }),
                infix(left(93), just(Token::GreatherThan), |l, _, r, _| {
                    Expression::GreaterThan(Box::new(l), Box::new(r))
                }),
                infix(left(93), just(Token::LessThanEqual), |l, _, r, _| {
                    Expression::LessThanOrEqual(Box::new(l), Box::new(r))
                }),
                infix(left(93), just(Token::GreaterThanEquals), |l, _, r, _| {
                    Expression::GreaterThanOrEqual(Box::new(l), Box::new(r))
                }),
                infix(
                    left(93),
                    just(Token::LessThanEqualsGreaterThan),
                    |l, _, r, _| Expression::Spaceship(Box::new(l), Box::new(r)),
                ),
                // EQUALITY
                infix(left(94), just(Token::EqualsEquals), |l, _, r, _| {
                    Expression::Equal(Box::new(l), Box::new(r))
                }),
                infix(left(94), just(Token::ExclamationEquals), |l, _, r, _| {
                    Expression::NotEqual(Box::new(l), Box::new(r))
                }),
                infix(left(94), just(Token::LessThanGreaterThan), |l, _, r, _| {
                    Expression::Inequal(Box::new(l), Box::new(r))
                }),
                infix(left(94), just(Token::EqualsEqualsEquals), |l, _, r, _| {
                    Expression::StrictEqual(Box::new(l), Box::new(r))
                }),
                infix(
                    left(94),
                    just(Token::ExclamationEqualsEquals),
                    |l, _, r, _| Expression::StrictNotEqual(Box::new(l), Box::new(r)),
                ),
                // BITWISE AND
                infix(left(93), just(Token::Ampersand), |l, _, r, _| {
                    Expression::BitwiseAnd(Box::new(l), Box::new(r))
                }),
                // BITWISE EXC
                infix(left(92), just(Token::Caret), |l, _, r, _| {
                    Expression::BitwiseExc(Box::new(l), Box::new(r))
                }),
                // BITWISE INC
            ))
            .pratt((
                infix(left(91), just(Token::Bar), |l, _, r, _| {
                    Expression::BitwiseInc(Box::new(l), Box::new(r))
                }),
                infix(left(90), just(Token::AmpersandAmpersand), |l, _, r, _| {
                    Expression::And(Box::new(l), Box::new(r))
                }),
                infix(left(89), just(Token::BarBar), |l, _, r, _| {
                    Expression::Or(Box::new(l), Box::new(r))
                }),
                infix(left(88), just(Token::QuestionQuestion), |l, _, r, _| {
                    Expression::Coalesce(Box::new(l), Box::new(r))
                }),
            ))
        })
        .labelled("Expression")
        .boxed()
    }

    pub fn list_parser<I>() -> impl Parser<'a, I, Vec<Self>, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Self::parser()
            .separated_by(just(Token::Comma))
            .at_least(1)
            .collect()
    }
}
