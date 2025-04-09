use crate::parser::atoms::intrinsic::Intrinsic;
use crate::parser::statements::Statement;
use crate::parser::variables::Variable;
use crate::parser::BoxedParser;

use super::anonymous_function_expression::AnonymousFunctionCreationExpression;
use super::array_creation_expression::ArrayCreationExpression;
use super::byref_assignment_expression::ByrefAssigmentExpression;
use super::class_constant_access_expression::ClassConstantAccessExpression;
use super::constant_access_expression::ConstantAccessExpression;
use super::decrement_expression::DecrementExpression;
use super::include_expression::IncludeExpression;
use super::include_once_expression::IncludeOnceExpression;
use super::increment_expression::InscrementExpression;
use super::literal_expression::LiteralExpression;
use super::object_creation_expression::ObjectCreationExpression;
use super::require_expression::RequireExpression;
use super::require_once_expression::RequireOnceExpression;
use super::reserved_word_expression::ReservedWordExpression;
use super::Expression;
use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimaryExpression<'a> {
    Variable(Box<Variable<'a>>),
    ClassConstantAccess(ClassConstantAccessExpression<'a>),
    ConstantAccess(ConstantAccessExpression<'a>),
    Literal(LiteralExpression<'a>),
    ArrayCreation(ArrayCreationExpression<'a>),
    Intrinsic(Box<Intrinsic<'a>>),
    AnonymousFunctionCreation(AnonymousFunctionCreationExpression<'a>),
    ObjectCreation(Box<ObjectCreationExpression<'a>>), // TODO
    Increment(Box<InscrementExpression<'a>>),
    Decrement(Box<DecrementExpression<'a>>),
    ByrefAssignment(Box<ByrefAssigmentExpression<'a>>),
    ShellCommand, // TODO

    // TODO: Move to Expression
    ReservedWord(ReservedWordExpression),
    Require(Box<RequireExpression<'a>>),
    RequireOnce(Box<RequireOnceExpression<'a>>),
    Include(Box<IncludeExpression<'a>>),
    IncludeOnce(Box<IncludeOnceExpression<'a>>),
    Expression(Box<Expression<'a>>),
}

impl<'a> PrimaryExpression<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let variable =
            Variable::parser(expression_parser.clone()).map(|v| Self::Variable(Box::new(v)));

        let class_constant_access =
            ClassConstantAccessExpression::parser().map(Self::ClassConstantAccess);

        let constant_access = ConstantAccessExpression::parser().map(Self::ConstantAccess);

        let literal = LiteralExpression::parser().map(Self::Literal);

        let array_creation =
            ArrayCreationExpression::parser(expression_parser.clone()).map(Self::ArrayCreation);

        let intrinsic =
            Intrinsic::parser(expression_parser.clone()).map(|v| Self::Intrinsic(Box::new(v)));

        let anonymous_function_creation =
            AnonymousFunctionCreationExpression::parser(expression_parser.clone())
                .map(Self::AnonymousFunctionCreation);

        let object_creation = ObjectCreationExpression::parser(expression_parser.clone())
            .map(|v| Self::ObjectCreation(Box::new(v)));

        let increment = InscrementExpression::parser(expression_parser.clone())
            .map(|v| Self::Increment(Box::new(v)));

        let decrement = DecrementExpression::parser(expression_parser.clone())
            .map(|v| Self::Decrement(Box::new(v)));

        let byref_assignment = ByrefAssigmentExpression::parser(expression_parser.clone())
            .map(|v| Self::ByrefAssignment(Box::new(v)));

        let reserved_word = ReservedWordExpression::parser().map(Self::ReservedWord);

        let require_once = RequireOnceExpression::parser(expression_parser.clone())
            .map(|e| Self::RequireOnce(Box::new(e)));

        let require = RequireExpression::parser(expression_parser.clone())
            .map(|e| Self::Require(Box::new(e)));

        let include = IncludeExpression::parser(expression_parser.clone())
            .map(|e| Self::Include(Box::new(e)));

        let include_once = IncludeOnceExpression::parser(expression_parser.clone())
            .map(|e| Self::IncludeOnce(Box::new(e)));

        let expression = just(Token::OpenParen)
            .ignore_then(expression_parser)
            .then_ignore(just(Token::CloseParen))
            .map(|e| Self::Expression(Box::new(e)));

        choice((
            anonymous_function_creation,
            class_constant_access,
            constant_access,
            literal,
            array_creation,
            object_creation,
            intrinsic,
            increment,
            decrement,
            byref_assignment,
            reserved_word,
            require_once,
            require,
            include,
            include_once,
            variable,
            expression,
        ))
        .labelled("Expression")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<PrimaryExpression, ()> {
        let tokens = tokenize(src);

        PrimaryExpression::parser(Expression::parser())
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn class_constant_access() {
        assert!(matches!(
            parse(r#"self::Hello"#),
            Ok(PrimaryExpression::ClassConstantAccess(_))
        ));
    }

    #[test]
    fn constant_access() {
        assert!(matches!(
            parse(r#"Hello"#),
            Ok(PrimaryExpression::ConstantAccess(_))
        ));
    }

    #[test]
    fn literal() {
        assert!(matches!(
            parse(r#""Heyyy""#),
            Ok(PrimaryExpression::Literal(_))
        ));
    }

    #[test]
    fn array_creation() {
        assert!(matches!(
            parse(r#"array()"#),
            Ok(PrimaryExpression::ArrayCreation(_))
        ));
    }

    #[test]
    fn object_creation() {
        assert!(matches!(
            parse(r#"new Test()"#),
            Ok(PrimaryExpression::ObjectCreation(_))
        ));
    }

    #[test]
    fn intrinsic() {
        assert!(matches!(
            parse(r#"die()"#),
            Ok(PrimaryExpression::Intrinsic(_))
        ));
    }

    #[test]
    fn anonymous_function_creation() {
        assert!(matches!(
            parse(r#"function ()"#),
            Ok(PrimaryExpression::AnonymousFunctionCreation(_))
        ));
    }

    #[test]
    fn increment() {
        assert!(matches!(
            parse(r#"++$test"#),
            Ok(PrimaryExpression::Increment(_))
        ));

        assert!(matches!(
            parse(r#"$test++"#),
            Ok(PrimaryExpression::Increment(_))
        ));
    }

    #[test]
    fn decrement() {
        assert!(matches!(
            parse(r#"--$test"#),
            Ok(PrimaryExpression::Decrement(_))
        ));

        assert!(matches!(
            parse(r#"$test--"#),
            Ok(PrimaryExpression::Decrement(_))
        ));
    }

    #[test]
    fn byref_assignment() {
        assert!(matches!(
            parse(r#"$hello = &$world"#),
            Ok(PrimaryExpression::ByrefAssignment(_)),
        ));
    }

    #[test]
    fn expression() {
        assert!(matches!(
            parse(r#"($hello)"#),
            Ok(PrimaryExpression::Expression(_)),
        ));
    }
}
