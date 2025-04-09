use chumsky::input::ValueInput;
use chumsky::pratt::*;
use chumsky::prelude::*;
use chumsky::Parser;
use phprs_lexer::Token;
use primary_expression::PrimaryExpression;

use super::statements::Statement;
use super::BoxedParser;

pub mod anonymous_function_expression;
pub mod argument_expression;
pub mod array_creation_expression;
pub mod byref_assignment_expression;
pub mod callable_expression;
pub mod class_constant_access_expression;
pub mod constant_access_expression;
pub mod decrement_expression;
pub mod dereferencable_expression;
pub mod include_expression;
pub mod include_once_expression;
pub mod increment_expression;
pub mod literal_expression;
pub mod member_access_expression;
pub mod member_call_expression;
pub mod object_creation_expression;
pub mod primary_expression;
pub mod require_expression;
pub mod require_once_expression;
pub mod reserved_word_expression;
pub mod scoped_property_access_expression;

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
    Conditional(Box<Self>, Box<Option<Expression<'a>>>, Box<Self>),
    Assignment(Box<Self>, Box<Self>),
}

impl<'a> Expression<'a> {
    // pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> BoxedParser<'a, I, Self>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        recursive(|expression_parser| {
            let atom =
                PrimaryExpression::parser(statement_parser, expression_parser.clone().boxed())
                    .map(Expression::Primary);

            let conditional_infix = just(Token::Question)
                .ignore_then(expression_parser.or_not())
                .then_ignore(just(Token::Colon));

            atom.pratt((
                // CLONE
                prefix(99, just(Token::CloneKeyword), |_, r, _| {
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
                infix(
                    left(87),
                    conditional_infix,
                    |l, e: Option<Expression<'a>>, r, _| {
                        Expression::Conditional(Box::new(l), Box::new(e), Box::new(r))
                    },
                ),
                infix(left(86), just(Token::Equals), |l, _, r, _| {
                    Expression::Assignment(Box::new(l), Box::new(r))
                }),
            ))
        })
        .labelled("Expression")
        .boxed()
    }

    pub fn list_parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Vec<Self>, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Self::parser(statement_parser)
            .separated_by(just(Token::Comma))
            .at_least(1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<Expression, ()> {
        let tokens = tokenize(src);

        let statement_parser = Statement::parser().boxed();
        Expression::parser(statement_parser)
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn clone() {
        assert!(matches!(parse(r#"clone $test"#), Ok(Expression::Clone(_))));
        assert!(matches!(
            parse(r#"clone ($test + 4)"#),
            Ok(Expression::Clone(_))
        ));
    }

    #[test]
    fn add() {
        let res = parse(r#"5 + 8"#);
        assert!(matches!(res, Ok(Expression::Add(_, _))));
    }

    #[test]
    fn power() {
        assert!(matches!(
            parse(r#"$test ** 8"#),
            Ok(Expression::Power(_, _))
        ));
    }

    #[test]
    fn greater_than_or_equal() {
        assert!(matches!(
            parse(r#"$test >= 8"#),
            Ok(Expression::GreaterThanOrEqual(_, _))
        ));

        assert!(matches!(
            parse(r#"$test >= +8"#),
            Ok(Expression::GreaterThanOrEqual(_, _))
        ));

        assert!(matches!(
            parse(r#"$test - 8 >= +8"#),
            Ok(Expression::GreaterThanOrEqual(_, _))
        ));
    }

    #[test]
    fn conditional() {
        assert!(matches!(
            parse(r#"$hey ? 4 : 8"#),
            Ok(Expression::Conditional(_, _, _))
        ));

        assert!(matches!(
            parse(r#"$hey ? 4 : 8 + 8"#),
            Ok(Expression::Conditional(_, _, _))
        ));

        assert!(matches!(
            parse(r#"$hey ?: true"#),
            Ok(Expression::Conditional(_, _, _))
        ));
    }

    #[test]
    fn assignment() {
        assert!(matches!(
            parse(r#"$hey = 8"#),
            Ok(Expression::Assignment(_, _))
        ));

        assert!(matches!(
            parse(r#"$hey = 8 + 3"#),
            Ok(Expression::Assignment(_, _))
        ));
    }
}
