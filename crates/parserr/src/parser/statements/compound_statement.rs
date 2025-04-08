use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CompoundStatement<'a> {
    pub statements: Vec<Statement<'a>>,
}

impl<'a> CompoundStatement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        statement_parser
            .repeated()
            .collect()
            .delimited_by(just(Token::OpenBrace), just(Token::CloseBrace))
            .map(|statements| CompoundStatement { statements })
            .labelled("CompoundStatement")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{statements::expression_statement::ExpressionStatement, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<CompoundStatement, ()> {
        let tokens = tokenize(src);

        CompoundStatement::parser(Statement::parser().boxed())
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn empty() {
        let res = parse(r#"{}"#);
        assert_eq!(res, Ok(CompoundStatement { statements: vec![] }));
    }

    #[test]
    fn simple() {
        let res = parse(r#"{;}"#);
        assert_eq!(
            res,
            Ok(CompoundStatement {
                statements: vec![Statement::Expression(ExpressionStatement {
                    expression: None
                })]
            })
        );
    }
}
