use crate::parser::{variables::Variable, BoxedParser};
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ByrefAssigmentExpression<'a> {
    key: Variable<'a>,
    value: Variable<'a>,
}

impl<'a> ByrefAssigmentExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Variable::parser(expression_parser.clone())
            .then_ignore(just(Token::Equals).then(just(Token::Ampersand)))
            .then(Variable::parser(expression_parser))
            .map(|(key, value)| Self { key, value })
            .labelled("ByrefAssignmentExpression")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{statements::Statement, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<ByrefAssigmentExpression, ()> {
        let tokens = tokenize(src);

        ByrefAssigmentExpression::parser(Expression::parser(Statement::parser().boxed()))
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"$test = &$hello"#);
        assert!(matches!(
            res,
            Ok(ByrefAssigmentExpression {
                key: Variable::Callable(_),
                value: Variable::Callable(_)
            })
        ));
    }
}
