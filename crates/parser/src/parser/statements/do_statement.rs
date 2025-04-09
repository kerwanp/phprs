use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DoStatement<'a> {
    statement: Statement<'a>,
    expression: Expression<'a>,
}

impl<'a> DoStatement<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::DoKeyword)
            .ignore_then(statement_parser.clone())
            .then_ignore(just(Token::WhileKeyword))
            .then_ignore(just(Token::OpenParen))
            .then(Expression::parser(statement_parser))
            .then_ignore(just(Token::CloseParen))
            .then_ignore(just(Token::Semicolon))
            .map(|(statement, expression)| DoStatement {
                statement,
                expression,
            })
            .labelled("DoStatement")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        atoms::literal::{integer_literal::IntegerLiteral, Literal},
        expressions::{
            literal_expression::LiteralExpression, primary_expression::PrimaryExpression,
        },
        statements::expression_statement::ExpressionStatement,
        tokenize,
    };

    use super::*;

    fn parse(src: &str) -> Result<DoStatement, ()> {
        let token_stream = tokenize(src);

        DoStatement::parser(Statement::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn empty() {
        let res = parse(r#"do; while (5);"#);
        assert_eq!(
            res,
            Ok(DoStatement {
                statement: Statement::Expression(ExpressionStatement { expression: None }),
                expression: Expression::Primary(PrimaryExpression::Literal(LiteralExpression(
                    Literal::Integer(IntegerLiteral("5"))
                )))
            })
        );
    }

    // #[test]
    // fn simple() {
    //     let res = parse(r#"{;}"#);
    //     assert_eq!(
    //         res,
    //         Ok(CompoundStatement {
    //             statements: vec![Statement::Expression(ExpressionStatement {
    //                 expression: None
    //             })]
    //         })
    //     );
    // }
}
