use nom::{bytes::complete::is_not, character::complete::char, sequence::delimited, Parser};

use crate::{parser::util::ws, Error};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StringLiteral<'a>(&'a str);

impl<'a> StringLiteral<'a> {
    // TODO: Make this properly
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let build_string = is_not("\"");

        ws(delimited(char('"'), build_string, char('"')))
            .map(Self)
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn primary() {
        let variable = StringLiteral::parse(r#""Hello world!""#);
        assert_matches!(variable, Ok(("", StringLiteral("Hello world!"))));
    }
}
