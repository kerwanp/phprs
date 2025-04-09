use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use crate::parser::class::constructor_declaration::ConstructorDeclaration;
use crate::parser::class::destructor_declaration::DestructorDeclaration;
use crate::parser::class::method_declaration::MethodDeclaration;
use crate::parser::class::property_declaration::PropertyDeclaration;
use crate::parser::class::trait_use_clause::TraitUseClause;
use crate::parser::statements::Statement;
use crate::parser::BoxedParser;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TraitMemberDeclaration<'a> {
    PropertyDeclarationd(PropertyDeclaration<'a>),
    MethodDeclaration(MethodDeclaration<'a>),
    ConstructorDeclaration(ConstructorDeclaration<'a>),
    DestructorDeclaration(DestructorDeclaration<'a>),
    TraitUseClause(TraitUseClause<'a>),
}

impl<'a> TraitMemberDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let property_declaration =
            PropertyDeclaration::parser(statement_parser.clone()).map(Self::PropertyDeclarationd);
        let method_declaration =
            MethodDeclaration::parser(statement_parser.clone()).map(Self::MethodDeclaration);
        let constructor_declaration = ConstructorDeclaration::parser(statement_parser.clone())
            .map(Self::ConstructorDeclaration);
        let destructor_declaration =
            DestructorDeclaration::parser(statement_parser).map(Self::DestructorDeclaration);
        let trait_use_clause = TraitUseClause::parser().map(Self::TraitUseClause);

        choice((
            property_declaration,
            method_declaration,
            constructor_declaration,
            destructor_declaration,
            trait_use_clause,
        ))
        .labelled("TraitMemberDeclaration")
    }
}
