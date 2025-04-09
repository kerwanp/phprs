use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::expressions::Expression;
use crate::parser::variables::simple::SimpleVariable;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalDeclaration<'a>(Vec<SimpleVariable<'a>>);

impl<'a> GlobalDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::GlobalKeyword)
            .ignore_then(SimpleVariable::list_parser(
                Expression::parser(statement_parser).boxed(),
            ))
            .then_ignore(just(Token::Semicolon))
            .map(Self)
            .labelled("ConstDeclaration")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{atoms::name::variable_name::VariableName, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<GlobalDeclaration, ()> {
        let token_stream = tokenize(src);

        GlobalDeclaration::parser(Statement::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"global $hello;"#);
        assert_eq!(
            res,
            Ok(GlobalDeclaration(vec![SimpleVariable::Name(VariableName(
                "$hello"
            ))]))
        );
    }

    #[test]
    fn list() {
        let res = parse(r#"global $hello, $world;"#);
        assert_eq!(
            res,
            Ok(GlobalDeclaration(vec![
                SimpleVariable::Name(VariableName("$hello")),
                SimpleVariable::Name(VariableName("$world"))
            ]))
        );
    }
}
