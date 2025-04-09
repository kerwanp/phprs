pub mod atoms;
pub mod class;
pub mod expressions;
pub mod interface;
pub mod script;
pub mod statements;
pub mod variables;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::input::Stream;
use chumsky::input::ValueInput;
use chumsky::prelude::*;
use chumsky::Parser;
use phprs_lexer::Token;
use script::Script;

type BoxedParser<'a, I, O> = Boxed<'a, 'a, I, O, extra::Err<Rich<'a, Token<'a>>>>;

pub fn parse(content: &str) {
    let token_iter = phprs_lexer::lexer(content).map(|(tok, span)| match tok {
        Ok(tok) => (tok, span.into()),
        Err(_) => (Token::Unknown, span.into()),
    });

    let token_stream =
        Stream::from_iter(token_iter).map((0..content.len()).into(), |(t, s): (_, _)| (t, s));

    match Script::parser().parse(token_stream).into_result() {
        Ok(script) => println!("{:#?}", script),
        Err(errs) => {
            for err in errs {
                Report::build(ReportKind::Error, (), err.span().start)
                    .with_code(3)
                    .with_message(err.to_string())
                    .with_label(
                        Label::new(err.span().into_range())
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .eprint(Source::from(content))
                    .unwrap();
            }
        }
    }
}

pub fn tokenize(src: &str) -> impl chumsky::input::ValueInput<Token = Token, Span = SimpleSpan> {
    // ) -> chumsky::input::SpannedInput<Token, SimpleSpan, BoxedStream<(Token, SimpleSpan)>> {
    let token_iter = phprs_lexer::lexer(src)
        .filter(|(tok, _)| !matches!(tok, Ok(Token::EndOfFile)))
        .map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(_) => (Token::Unknown, span.into()),
        });

    Stream::from_iter(token_iter)
        .boxed()
        .map((0..src.len()).into(), |(t, s): (_, _)| (t, s))
}

pub trait Parseable<'a>: Sized {
    fn parser<I>() -> Boxed<'a, 'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>;
}
