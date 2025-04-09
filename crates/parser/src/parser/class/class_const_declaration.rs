use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::const_element::ConstElement;
use crate::parser::atoms::visibility_modifier::VisibilityModifier;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassConstDeclaration<'a> {
    visibility: Option<VisibilityModifier>,
    elements: Vec<ConstElement<'a>>,
}

impl<'a> ClassConstDeclaration<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let visiblity = VisibilityModifier::parser().or_not();

        visiblity
            .then_ignore(just(Token::ConstKeyword))
            .then(ConstElement::list_parser())
            .then_ignore(just(Token::Semicolon))
            .map(|(visibility, elements)| Self {
                visibility,
                elements,
            })
    }
}
