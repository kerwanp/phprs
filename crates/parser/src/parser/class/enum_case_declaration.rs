use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::atoms::name::Name;
use crate::parser::atoms::visibility_modifier::VisibilityModifier;
use crate::parser::expressions::Expression;
use crate::parser::statements::Statement;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnumCaseDeclaration<'a> {
    visibility: Option<VisibilityModifier>,
    name: Name<'a>,
    expression: Expression<'a>,
}

impl<'a> EnumCaseDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let visiblity = VisibilityModifier::parser().or_not();

        visiblity
            .then_ignore(just(Token::CaseKeyword))
            .then(Name::parser())
            .then_ignore(just(Token::Equals))
            .then(Expression::parser(statement_parser))
            .then_ignore(just(Token::Semicolon))
            .map(|((visibility, name), expression)| Self {
                visibility,
                name,
                expression,
            })
            .labelled("EnumCaseDeclaration")
    }
}
