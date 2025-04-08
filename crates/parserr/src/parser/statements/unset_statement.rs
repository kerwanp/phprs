use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::expressions::Expression;
use crate::parser::variables::Variable;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnsetStatement<'a> {
    variables: Vec<Variable<'a>>,
}

impl<'a> UnsetStatement<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::UnsetKeyword)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(Variable::list_parser(Expression::parser().boxed()))
            .then_ignore(just(Token::CloseParen))
            .then_ignore(just(Token::Semicolon))
            .map(|variables| UnsetStatement { variables })
            .labelled("UnsetStatement")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        atoms::name::variable_name::VariableName,
        variables::{callable::CallableVariable, simple::SimpleVariable},
    };

    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<UnsetStatement, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        UnsetStatement::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"unset($test);"#);
        assert_eq!(
            res,
            Ok(UnsetStatement {
                variables: vec![Variable::Callable(CallableVariable::Simple(
                    SimpleVariable::Name(VariableName("$test"))
                ))]
            })
        );
    }

    #[test]
    fn multiple() {
        let res = parse(r#"unset($hello, $world);"#);
        assert_eq!(
            res,
            Ok(UnsetStatement {
                variables: vec![
                    Variable::Callable(CallableVariable::Simple(SimpleVariable::Name(
                        VariableName("$hello")
                    ))),
                    Variable::Callable(CallableVariable::Simple(SimpleVariable::Name(
                        VariableName("$world")
                    )))
                ]
            })
        );
    }

    #[test]
    fn trailing() {
        let res = parse(r#"unset($hello, $world,);"#);
        assert_eq!(
            res,
            Ok(UnsetStatement {
                variables: vec![
                    Variable::Callable(CallableVariable::Simple(SimpleVariable::Name(
                        VariableName("$hello")
                    ))),
                    Variable::Callable(CallableVariable::Simple(SimpleVariable::Name(
                        VariableName("$world")
                    )))
                ]
            })
        );
    }
}
