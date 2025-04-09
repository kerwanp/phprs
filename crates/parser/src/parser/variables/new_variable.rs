use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use crate::parser::atoms::member_name::MemberName;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use crate::parser::{
    atoms::{name::qualified_name::QualifiedName, relative_scope::RelativeScope},
    variables::simple::SimpleVariable,
};

use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NewVariable<'a> {
    SimpleVariable(SimpleVariable<'a>),
    // TODO: Handle
    ArrayAccess(Box<NewVariable<'a>>, Option<Expression<'a>>),
    // NOTE: Deprecated
    OffsetAccess,
    Qualified(QualifiedName<'a>, SimpleVariable<'a>),
    Relative(RelativeScope, SimpleVariable<'a>),

    MemberAccess(Box<NewVariable<'a>>, MemberName<'a>),
    ColonAccess(Box<NewVariable<'a>>, SimpleVariable<'a>),
}

type PostfixOp<'a> = Box<dyn FnOnce(NewVariable<'a>) -> NewVariable<'a> + 'a>;

impl<'a> NewVariable<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let base = Self::base_parser(expression_parser.clone());
        let postfixes = Self::postfix_parser(expression_parser);

        base.foldl(postfixes.repeated(), |a, op: PostfixOp<'a>| op(a))
    }

    fn base_parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let simple_var =
            SimpleVariable::parser(expression_parser.clone()).map(NewVariable::SimpleVariable);

        let qualified = QualifiedName::parser()
            .then_ignore(just(Token::ColonColon))
            .then(SimpleVariable::parser(expression_parser.clone()))
            .map(|(qn, sv)| NewVariable::Qualified(qn, sv));

        let relative = RelativeScope::parser()
            .then_ignore(just(Token::ColonColon))
            .then(SimpleVariable::parser(expression_parser.clone()))
            .map(|(rs, sv)| NewVariable::Relative(rs, sv));

        choice((simple_var, qualified, relative))
    }

    fn postfix_parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, PostfixOp<'a>, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        choice((
            just(Token::ColonColon)
                .ignore_then(SimpleVariable::parser(expression_parser.clone()))
                .map(|member| {
                    Box::new(move |var: NewVariable<'a>| Self::ColonAccess(Box::new(var), member))
                        as PostfixOp<'a>
                }),
            just(Token::Arrow)
                .ignore_then(MemberName::parser(expression_parser.clone()))
                .map(|member| {
                    Box::new(move |var: NewVariable<'a>| {
                        NewVariable::MemberAccess(Box::new(var), member)
                    }) as PostfixOp<'a>
                }),
            just(Token::ColonColon)
                .ignore_then(SimpleVariable::parser(expression_parser))
                .map(|member| {
                    Box::new(move |_| NewVariable::SimpleVariable(member)) as PostfixOp<'a>
                }),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{statements::Statement, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<NewVariable, ()> {
        let token_stream = tokenize(src);

        NewVariable::parser(Expression::parser(Statement::parser().boxed()))
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn nested() {
        // let res = parse(r#"$test->test->hey->NOP"#);
        // println!("{:?}", res);
        // assert_eq!(false, true)
        // assert!(matches!(
        //     res,
        //     Ok(NewVariable::ColonColon(
        //         Box(NewVariable::SimpleVariable(_)),
        //         _
        //     ))
        // ))
    }

    #[test]
    fn member() {
        let res = parse(r#"$test->test"#);
        println!("{:?}", res);
        assert!(matches!(res, Ok(NewVariable::MemberAccess(_, _))))
    }

    #[test]
    fn simple() {
        let res = parse(r#"$test"#);
        assert!(matches!(res, Ok(NewVariable::SimpleVariable(_))))
    }

    #[test]
    fn qualified() {
        let res = parse(r#"Hey::$test"#);
        assert!(matches!(res, Ok(NewVariable::Qualified(_, _))))
    }

    #[test]
    fn relative() {
        let res = parse(r#"self::$test"#);
        assert!(matches!(res, Ok(NewVariable::Relative(_, _))))
    }
}
