use nom::{branch::alt, bytes::complete::tag, combinator::opt, sequence::delimited, Parser};

use super::{statement::Statement, util::ws, Error};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Script<'a> {
    statements: Vec<Statement<'a>>,
}

impl<'a> Script<'a> {
    // pub fn parser<'a>() -> impl Parser<&'a str, Script, Error<'a>> {
    //     let start = ws(alt((tag("<?php"), tag("<?="))));
    //     let end = ws(opt(tag("?>")));
    //
    //     tuple((start, end)).map(|_| Self {})
    // }

    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let start = ws(alt((tag("<?php"), tag("<?="))));
        let end = ws(opt(tag("?>")));
        let inner = Statement::parse_many;
        delimited(start, inner, end)
            .map(|statements| Self { statements })
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    pub fn start() {
        let test = Script::parse("<?php");
        assert_matches!(test, Ok(("", Script { statements: _ })));
    }

    #[test]
    pub fn with_end() {
        let test = Script::parse("<?php ?>");
        assert_matches!(test, Ok(("", Script { statements: _ })));
    }

    #[test]
    pub fn equal() {
        let test = Script::parse("<?= ?>");
        assert_matches!(test, Ok(("", Script { statements: _ })));
    }

    #[test]
    pub fn statements() {
        let test = Script::parse(
            "<?php
0b110;",
        );
        assert_matches!(test, Ok(("", Script { statements: _ })));
    }
}
