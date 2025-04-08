use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::expressions::Expression;
use crate::parser::variables::Variable;
use crate::parser::BoxedParser;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IssetIntrinsic<'a>(Vec<Variable<'a>>);

impl<'a> IssetIntrinsic<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::IssetKeyword)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(Variable::list_parser(expression_parser))
            .then_ignore(just(Token::CloseParen))
            .map(Self)
            .labelled("IssetIntrinsic")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        atoms::name::variable_name::VariableName,
        tokenize,
        variables::{callable::CallableVariable, simple::SimpleVariable},
    };

    use super::*;

    fn parse(src: &str) -> Result<IssetIntrinsic, ()> {
        let tokens = tokenize(src);

        IssetIntrinsic::parser(Expression::parser().boxed())
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"isset($test)"#);
        assert_eq!(
            res,
            Ok(IssetIntrinsic(vec![Variable::Callable(
                CallableVariable::Simple(SimpleVariable::Name(VariableName("$test")))
            )]))
        );
    }

    #[test]
    fn multiple_variable() {
        let res = parse(r#"isset($hello, $world)"#);
        assert_eq!(
            res,
            Ok(IssetIntrinsic(vec![
                Variable::Callable(CallableVariable::Simple(SimpleVariable::Name(
                    VariableName("$hello")
                ))),
                Variable::Callable(CallableVariable::Simple(SimpleVariable::Name(
                    VariableName("$world")
                )))
            ]))
        );
    }
}
