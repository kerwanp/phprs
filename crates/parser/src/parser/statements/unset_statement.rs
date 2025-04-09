use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::expressions::Expression;
use crate::parser::variables::Variable;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnsetStatement<'a> {
    variables: Vec<Variable<'a>>,
}

impl<'a> UnsetStatement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::UnsetKeyword)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(Variable::list_parser(Expression::parser(statement_parser)))
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
        tokenize,
        variables::{callable::CallableVariable, simple::SimpleVariable},
    };

    use super::*;

    fn parse(src: &str) -> Result<UnsetStatement, ()> {
        let token_stream = tokenize(src);

        UnsetStatement::parser(Statement::parser().boxed())
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
