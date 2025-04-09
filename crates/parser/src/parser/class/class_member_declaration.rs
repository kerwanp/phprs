use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use crate::parser::statements::Statement;
use crate::parser::BoxedParser;

use super::class_const_declaration::ClassConstDeclaration;
use super::constructor_declaration::ConstructorDeclaration;
use super::destructor_declaration::DestructorDeclaration;
use super::method_declaration::MethodDeclaration;
use super::property_declaration::PropertyDeclaration;
use super::trait_use_clause::TraitUseClause;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ClassMemberDeclaration<'a> {
    ClassConstDeclaration(ClassConstDeclaration<'a>),
    PropertyDeclaration(PropertyDeclaration<'a>),
    MethodDeclaration(MethodDeclaration<'a>),
    ConstructorDeclaration(ConstructorDeclaration<'a>),
    DestructorDeclaration(DestructorDeclaration<'a>),
    TraitUseClause(TraitUseClause<'a>),
}

impl<'a> ClassMemberDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let class_const_declaration =
            ClassConstDeclaration::parser().map(Self::ClassConstDeclaration);
        let property_declaration = PropertyDeclaration::parser().map(Self::PropertyDeclaration);
        let method_declaration =
            MethodDeclaration::parser(statement_parser.clone()).map(Self::MethodDeclaration);
        let constructor_declaration = ConstructorDeclaration::parser(statement_parser.clone())
            .map(Self::ConstructorDeclaration);
        let destructor_declaration =
            DestructorDeclaration::parser(statement_parser).map(Self::DestructorDeclaration);
        let trait_use_clause = TraitUseClause::parser().map(Self::TraitUseClause);

        choice((
            class_const_declaration,
            property_declaration,
            method_declaration,
            constructor_declaration,
            destructor_declaration,
            trait_use_clause,
        ))
        .labelled("ClassMemberDeclaration")
    }
}
