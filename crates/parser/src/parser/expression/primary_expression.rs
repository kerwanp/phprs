use nom::{branch::alt, Parser};

use crate::Error;

use super::{
    empty_intrinsic::EmptyIntrinsic, eval_intrinsic::EvalIntrinsic,
    literal_expression::LiteralExpression, Expression,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimaryExpression<'a> {
    Variable,
    ClassConstantAccess,
    Literal(LiteralExpression<'a>),
    ArrayCreation,
    EmptyIntrinsic(EmptyIntrinsic<'a>),
    EvalIntrinsic(EvalIntrinsic<'a>),
    ExitIntrinsic,
    IssetIntrinsic,
    AnonymousFunctionCreation,
    Increment,
    Decrement,
    ByrefAssignment,
    ShellCommand,
    Expression(Expression<'a>), // TODO
}

impl<'a> PrimaryExpression<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let literal = LiteralExpression::parse.map(Self::Literal);
        let empty_intrinsic = EmptyIntrinsic::parse.map(Self::EmptyIntrinsic);
        let eval_intrinsic = EvalIntrinsic::parse.map(Self::EvalIntrinsic);

        alt((literal, empty_intrinsic, eval_intrinsic)).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn literal() {
        let variable = PrimaryExpression::parse("0b11");
        assert_matches!(variable, Ok((_, PrimaryExpression::Literal(_))));
    }

    #[test]
    fn empty_intrinsic() {
        let variable = PrimaryExpression::parse(r#"empty("hello")"#);
        assert_matches!(variable, Ok((_, PrimaryExpression::EmptyIntrinsic(_))));
    }

    #[test]
    fn eval_intrinsic() {
        let variable = PrimaryExpression::parse(r#"eval("hello")"#);
        assert_matches!(variable, Ok((_, PrimaryExpression::EvalIntrinsic(_))));
    }
}
