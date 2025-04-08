use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::expressions::Expression;
use crate::parser::variables::simple::SimpleVariable;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalDeclaration<'a>(Vec<SimpleVariable<'a>>);

impl<'a> GlobalDeclaration<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::GlobalKeyword)
            .ignore_then(SimpleVariable::list_parser(Expression::parser().boxed()))
            .then_ignore(just(Token::Semicolon))
            .map(Self)
            .labelled("ConstDeclaration")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::atoms::name::variable_name::VariableName;

    use super::*;
    use chumsky::input::Stream;

    fn parse(src: &str) -> Result<GlobalDeclaration, ()> {
        let token_iter = phprs_lexer::lexer(src)
            .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
            .map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Unknown, span.into()),
            });

        let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

        GlobalDeclaration::parser()
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
