pub mod empty_intrinsic;
pub mod eval_intrinsic;
pub mod exit_intrinsic;
pub mod isset_intrinsic;

use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, span::SimpleSpan, Parser};

use empty_intrinsic::EmptyIntrinsic;
use eval_intrinsic::EvalIntrinsic;
use exit_intrinsic::ExitIntrinsic;
use isset_intrinsic::IssetIntrinsic;
use phprs_lexer::Token;

use crate::parser::expressions::Expression;
use crate::parser::BoxedParser;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Intrinsic<'a> {
    Empty(EmptyIntrinsic<'a>),
    Eval(EvalIntrinsic<'a>),
    Exit(ExitIntrinsic<'a>),
    Isset(IssetIntrinsic<'a>),
}

impl<'a> Intrinsic<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let empty = EmptyIntrinsic::parser(expression_parser.clone()).map(Self::Empty);
        let eval = EvalIntrinsic::parser(expression_parser.clone()).map(Self::Eval);
        let exit = ExitIntrinsic::parser(expression_parser.clone()).map(Self::Exit);
        let isset = IssetIntrinsic::parser(expression_parser).map(Self::Isset);

        choice((empty, eval, exit, isset)).labelled("Intrinsic")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{statements::Statement, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<Intrinsic, ()> {
        let token_stream = tokenize(src);

        Intrinsic::parser(Expression::parser(Statement::parser()))
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn empty() {
        let res = parse(r#"empty(5)"#);
        assert!(matches!(res, Ok(Intrinsic::Empty(_))))
    }

    #[test]
    fn eval() {
        let res = parse(r#"eval(5)"#);
        assert!(matches!(res, Ok(Intrinsic::Eval(_))))
    }

    #[test]
    fn exit() {
        let res = parse(r#"exit"#);
        assert!(matches!(res, Ok(Intrinsic::Exit(_))))
    }

    #[test]
    fn isset() {
        let res = parse(r#"isset($test)"#);
        assert!(matches!(res, Ok(Intrinsic::Isset(_))))
    }
}
