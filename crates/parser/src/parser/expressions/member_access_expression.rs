use crate::parser::atoms::member_name::MemberName;
use crate::parser::BoxedParser;
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

use super::{dereferencable_expression::DereferencableExpression, Expression};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MemberAccessExpression<'a>(pub DereferencableExpression<'a>, pub MemberName<'a>);

impl<'a> MemberAccessExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        DereferencableExpression::parser(expression_parser.clone())
            .then_ignore(just(Token::Arrow))
            .then(MemberName::parser(expression_parser))
            .map(|(expression, name)| Self(expression, name))
            .labelled("MemberAccessExpression")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{statements::Statement, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<MemberAccessExpression, ()> {
        let token_stream = tokenize(src);

        let statement_parser = Statement::parser().boxed();
        let expression_parser = Expression::parser(statement_parser).boxed();
        MemberAccessExpression::parser(expression_parser.clone())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#""heeey"->hello"#);
        assert!(matches!(res, Ok(MemberAccessExpression(_, _))));
    }
}
