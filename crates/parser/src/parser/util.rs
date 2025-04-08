use nom::{character::complete::multispace0, sequence::delimited, Parser};

use crate::Error;

pub fn ws<'a, F, O>(inner: F) -> impl Parser<&'a str, O, Error<'a>>
where
    F: Parser<&'a str, O, Error<'a>>,
{
    delimited(multispace0, inner, multispace0)
}
