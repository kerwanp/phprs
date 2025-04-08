use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::property_element::PropertyElement;
use crate::parser::atoms::property_modifier::PropertyModifier;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PropertyDeclaration<'a> {
    modifier: PropertyModifier,
    elements: Vec<PropertyElement<'a>>,
}

impl<'a> PropertyDeclaration<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        PropertyModifier::parser()
            .then(PropertyElement::list_parser())
            .then_ignore(just(Token::Semicolon))
            .map(|(modifier, elements)| Self { modifier, elements })
    }
}
