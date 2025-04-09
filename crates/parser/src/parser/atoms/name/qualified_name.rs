use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use super::namespace_name::NamespaceName;
use super::Name;

// TODO: Check what is `namespace \App\`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamespaceNameAsPrefix<'a> {
    namespace: Option<NamespaceName<'a>>,
    global: bool,
}

impl<'a> NamespaceNameAsPrefix<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        // `\`
        let empty = just(Token::Backslack).map(|_| Self {
            global: true,
            namespace: None,
        });

        let simple = just(Token::Backslack)
            .or_not()
            .map(|t| t.is_some())
            .then(NamespaceName::parser())
            .then_ignore(just(Token::Backslack))
            .map(|(global, namespace)| Self {
                global,
                namespace: Some(namespace),
            });

        let with_keyword = just(Token::NamespaceKeyword)
            .ignore_then(
                just(Token::Backslack)
                    .ignore_then(NamespaceName::parser())
                    .or_not(),
            )
            .then_ignore(just(Token::Backslack))
            .map(|namespace| Self {
                global: true,
                namespace,
            });

        empty.or(simple).or(with_keyword)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QualifiedName<'a> {
    namespace: Option<NamespaceNameAsPrefix<'a>>,
    name: Name<'a>,
}

// TODO: Should be a Token
// Test\Hey does not work
impl<'a> QualifiedName<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        NamespaceNameAsPrefix::parser()
            .or_not()
            .then(Name::parser())
            .map(|(namespace, name)| Self { namespace, name })
    }
}
