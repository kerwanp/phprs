use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::atoms::name::namespace_name::NamespaceName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamespaceUseDeclaration<'a> {
    namespace_name: NamespaceName<'a>,
}

impl<'a> NamespaceUseDeclaration<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::UseKeyword)
            .ignore_then(NamespaceName::parser())
            .then_ignore(just(Token::Semicolon))
            .map(|namespace_name| NamespaceUseDeclaration { namespace_name })
            .labelled("NamespaceUseDeclaration")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<NamespaceUseDeclaration, ()> {
        let token_stream = tokenize(src);

        NamespaceUseDeclaration::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"use App\Test;"#);
        assert_eq!(
            res,
            Ok(NamespaceUseDeclaration {
                namespace_name: NamespaceName(vec!["App", "Test"])
            })
        );
    }

    #[test]
    fn no_colon_fail() {
        let res = parse(r#"use App"#);
        assert_eq!(res, Err(()));
    }

    #[test]
    fn empty_namespace_name_fail() {
        let res = parse(r#"use ;"#);
        assert_eq!(res, Err(()));
    }
}
