pub mod callable;
pub mod new_variable;
pub mod simple;

use crate::parser::BoxedParser;
use callable::CallableVariable;
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;
use simple::SimpleVariable;

use super::{
    atoms::{member_name::MemberName, scope_resolution_qualifier::ScopeResolutionQualifier},
    expressions::{
        argument_expression::ArgumentExpression,
        dereferencable_expression::DereferencableExpression, Expression,
    },
};

type PostfixOp<'a> = Box<dyn FnOnce(Variable<'a>) -> Variable<'a> + 'a>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Variable<'a> {
    Callable(CallableVariable<'a>),

    // TODO: foldl makes it harder to navigate the AST tree
    ScopedPropertryAccessExpression(ScopeResolutionQualifier<'a>, SimpleVariable<'a>), // TODO
    MemberAccessExpression(DereferencableExpression<'a>, MemberName<'a>),
    MemberCallExpression(DereferencableExpression<'a>, Vec<ArgumentExpression<'a>>),
}

impl<'a> Variable<'a> {
    pub fn postfix_parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, PostfixOp<'a>, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let member_access_postfix = just(Token::Arrow)
            .ignore_then(MemberName::parser(expression_parser.clone()))
            .map(|member| {
                Box::new(move |prev: Variable<'a>| {
                    Variable::MemberAccessExpression(
                        DereferencableExpression::Variable(Box::new(prev)),
                        member,
                    )
                }) as PostfixOp<'a>
            });

        let member_call_postfix = just(Token::OpenParen)
            .ignore_then(ArgumentExpression::list_parser(expression_parser.clone()))
            .then_ignore(just(Token::CloseParen))
            .map(|arguments| {
                Box::new(move |prev: Variable<'a>| {
                    Variable::MemberCallExpression(
                        DereferencableExpression::Variable(Box::new(prev)),
                        arguments,
                    )
                }) as PostfixOp<'a>
            });

        choice((member_access_postfix, member_call_postfix))
    }

    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let scoped = ScopeResolutionQualifier::parser()
            .then_ignore(just(Token::ColonColon))
            .then(SimpleVariable::parser(expression_parser.clone()))
            .map(|(qualifier, property)| {
                Variable::ScopedPropertryAccessExpression(qualifier, property)
            });

        let callable = CallableVariable::parser(expression_parser.clone()).map(Self::Callable);

        let base = choice((scoped, callable));

        let postfix = Self::postfix_parser(expression_parser);

        base.foldl(postfix.repeated(), |a, b| b(a))
            .labelled("Variable")
    }

    pub fn list_parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Vec<Self>, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Self::parser(expression_parser)
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .at_least(1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{statements::Statement, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<Variable, ()> {
        let token_stream = tokenize(src);

        Variable::parser(Expression::parser(Statement::parser()))
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"$test"#);
        assert!(matches!(res, Ok(Variable::Callable(_))));
    }

    #[test]
    fn method_access() {
        let res = parse(r#"$test->hello"#);
        assert!(matches!(res, Ok(Variable::MemberAccessExpression(_, _))));
        assert!(matches!(res, Ok(Variable::MemberAccessExpression(_, _))));
    }
}
