#![feature(impl_trait_in_bindings)]

pub mod parser;

use chumsky::input::Stream;
use chumsky::prelude::*;
use chumsky::Parser;
use parser::script::Script;
use phprs_lexer::Token;

pub fn parse(content: &str) -> Result<Script<'_>, Vec<Rich<'_, Token<'_>>>> {
    let token_iter = phprs_lexer::lexer(content).map(|(tok, span)| match tok {
        Ok(tok) => (tok, span.into()),
        Err(_) => (Token::Unknown, span.into()),
    });

    let token_stream =
        Stream::from_iter(token_iter).map((0..content.len()).into(), |(t, s): (_, _)| (t, s));

    Script::parser().parse(token_stream).into_result()
}
