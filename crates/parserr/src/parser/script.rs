use super::statements::Statement;
use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

#[derive(Debug)]
pub struct Script<'a> {
    statements: Vec<Statement<'a>>,
}

impl<'a> Script<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Script<'a>, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        just(Token::ScriptSectionStartTag)
            .ignore_then(Statement::list_parser(Statement::parser().boxed()))
            .then_ignore(just(Token::EndOfFile))
            .map(|statements| Script { statements })
    }
}
