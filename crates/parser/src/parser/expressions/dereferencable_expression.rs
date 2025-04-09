use crate::parser::atoms::literal::string_literal::StringLiteral;
use crate::parser::variables::Variable;
use crate::parser::BoxedParser;
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

use super::array_creation_expression::ArrayCreationExpression;
use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DereferencableExpression<'a> {
    // NOTE: foldl from Variable
    Variable(Box<Variable<'a>>),
    Expression(Expression<'a>),
    ArrayCreation(ArrayCreationExpression<'a>),
    StringLiteral(StringLiteral<'a>),
}

impl<'a> DereferencableExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let expression = just(Token::OpenParen)
            .ignore_then(expression_parser.clone())
            .then_ignore(just(Token::CloseParen))
            .map(Self::Expression);
        let array_creation =
            ArrayCreationExpression::parser(expression_parser).map(Self::ArrayCreation);
        let string_literal = StringLiteral::parser().map(Self::StringLiteral);

        choice((array_creation, string_literal, expression)).labelled("DereferencableExpression")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{statements::Statement, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<DereferencableExpression, ()> {
        let token_stream = tokenize(src);

        let statement_parser = Statement::parser().boxed();
        DereferencableExpression::parser(Expression::parser(statement_parser))
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn expression() {
        assert!(matches!(
            parse(r#"(4 + 8)"#),
            Ok(DereferencableExpression::Expression(_))
        ));
    }

    #[test]
    fn array_creation() {
        assert!(matches!(
            parse(r#"array()"#),
            Ok(DereferencableExpression::ArrayCreation(_))
        ));
    }

    #[test]
    fn string_literal() {
        assert!(matches!(
            parse(r#""Hello world""#),
            Ok(DereferencableExpression::StringLiteral(_))
        ));
    }
}
