use crate::parser::{
    atoms::name::variable_name::VariableName, expressions::Expression, BoxedParser,
};
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SimpleVariable<'a> {
    Name(VariableName<'a>),
    Simple(Box<SimpleVariable<'a>>),
    Expression(Expression<'a>),
}

impl<'a> SimpleVariable<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        recursive(|simple_variable| {
            // $variable
            let name = VariableName::parser().map(SimpleVariable::Name);

            // $$variable
            let simple = just(Token::Dollar)
                .ignore_then(simple_variable)
                .map(|sa| SimpleVariable::Simple(Box::new(sa)));

            let compound = choice((
                just(Token::DollarOpenBrace).ignored(),
                just(Token::Dollar).then(just(Token::OpenBrace)).ignored(),
            ))
            .ignore_then(expression_parser)
            .then_ignore(just(Token::CloseBrace))
            .map(SimpleVariable::Expression);

            choice((name, simple, compound))
        })
        .labelled("Simple variable")
    }

    pub fn list_parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Vec<Self>, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Self::parser(expression_parser)
            .separated_by(just(Token::Comma))
            .at_least(1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        atoms::literal::{integer_literal::IntegerLiteral, Literal},
        expressions::{
            literal_expression::LiteralExpression, primary_expression::PrimaryExpression,
        },
        tokenize,
    };

    use super::*;

    fn parse(src: &str) -> Result<SimpleVariable, ()> {
        let token_stream = tokenize(src);

        SimpleVariable::parser(Expression::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn name() {
        let res = parse(r#"$test"#);
        assert_eq!(res, Ok(SimpleVariable::Name(VariableName("$test"))));
    }

    #[test]
    fn simple() {
        let res = parse(r#"$$test"#);
        assert_eq!(
            res,
            Ok(SimpleVariable::Simple(Box::new(SimpleVariable::Name(
                VariableName("$test")
            ))))
        );
    }

    #[test]
    fn compound() {
        let res = parse(r#"${5}"#);
        assert_eq!(
            res,
            Ok(SimpleVariable::Expression(Expression::Primary(
                PrimaryExpression::Literal(LiteralExpression(Literal::Integer(IntegerLiteral(
                    "5"
                ))))
            )))
        );
    }

    #[test]
    fn compound_with_space() {
        let res = parse(r#"$ { 5 }"#);
        assert_eq!(
            res,
            Ok(SimpleVariable::Expression(Expression::Primary(
                PrimaryExpression::Literal(LiteralExpression(Literal::Integer(IntegerLiteral(
                    "5"
                ))))
            )))
        );
    }
}
