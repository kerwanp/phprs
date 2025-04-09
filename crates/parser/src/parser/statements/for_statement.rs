use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForStatement<'a> {
    initializer: Vec<Expression<'a>>,
    control: Vec<Expression<'a>>,
    end: Vec<Expression<'a>>,
    statements: Vec<Statement<'a>>,
}

impl<'a> ForStatement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let expression_group = Expression::parser()
            .separated_by(just(Token::Comma))
            .collect();
        let head = just(Token::ForKeyword)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(expression_group.clone())
            .then_ignore(just(Token::Semicolon))
            .then(expression_group.clone())
            .then_ignore(just(Token::Semicolon))
            .then(expression_group)
            .then_ignore(just(Token::CloseParen));

        let body1 = statement_parser.clone().map(|s| vec![s]);
        let body2 = just(Token::Colon)
            .ignore_then(Statement::list_parser(statement_parser))
            .then_ignore(just(Token::EndForKeyword))
            .then_ignore(just(Token::Semicolon));

        head.then(body1.or(body2))
            .map(|(((initializer, control), end), statements)| ForStatement {
                initializer,
                control,
                end,
                statements,
            })
            .labelled("ForStatement")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{statements::expression_statement::ExpressionStatement, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<ForStatement, ()> {
        let token_stream = tokenize(src);

        ForStatement::parser(Statement::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"for (;;);"#);
        assert_eq!(
            res,
            Ok(ForStatement {
                initializer: vec![],
                control: vec![],
                end: vec![],
                statements: vec![Statement::Expression(ExpressionStatement {
                    expression: None
                })]
            })
        );
    }

    #[test]
    fn keyword() {
        let res = parse(r#"for (;;):;endfor;"#);
        assert_eq!(
            res,
            Ok(ForStatement {
                initializer: vec![],
                control: vec![],
                end: vec![],
                statements: vec![Statement::Expression(ExpressionStatement {
                    expression: None
                })]
            })
        );
    }
}
