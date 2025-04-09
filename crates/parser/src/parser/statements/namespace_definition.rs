use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use phprs_lexer::Token;

use crate::parser::atoms::name::namespace_name::NamespaceName;
use crate::parser::BoxedParser;

use super::compound_statement::CompoundStatement;
use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NamespaceDefinition<'a> {
    Name(NamespaceName<'a>),
    Compound {
        name: Option<NamespaceName<'a>>,
        statement: CompoundStatement<'a>,
    },
}

impl<'a> NamespaceDefinition<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let name = just(Token::NamespaceKeyword)
            .ignore_then(NamespaceName::parser())
            .then_ignore(just(Token::Semicolon))
            .map(Self::Name);
        let compound = just(Token::NamespaceKeyword)
            .ignore_then(NamespaceName::parser().or_not())
            .then(CompoundStatement::parser(statement_parser))
            .map(|(name, statement)| Self::Compound { name, statement });

        name.or(compound)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<NamespaceDefinition, ()> {
        let tokens = tokenize(src);

        NamespaceDefinition::parser(Statement::parser().boxed())
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"namespace App\Test;"#);
        assert_eq!(
            res,
            Ok(NamespaceDefinition::Name(NamespaceName(vec![
                "App", "Test"
            ])))
        );
    }

    #[test]
    fn compound() {
        let res = parse(r#"namespace App {}"#);
        assert_eq!(
            res,
            Ok(NamespaceDefinition::Compound {
                name: Some(NamespaceName(vec!["App"])),
                statement: CompoundStatement { statements: vec![] }
            })
        );
    }
}
