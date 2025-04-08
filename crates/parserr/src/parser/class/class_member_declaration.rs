use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use crate::parser::statements::Statement;
use crate::parser::BoxedParser;

use super::class_const_declaration::ClassConstDeclaration;
use super::method_declaration::MethodDeclaration;
use super::property_declaration::PropertyDeclaration;
use super::trait_use_clause::TraitUseClause;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ClassMemberDeclaration<'a> {
    ClassConstDeclaration(ClassConstDeclaration<'a>),
    PropertyDeclaration(PropertyDeclaration<'a>),
    MethodDeclaration(MethodDeclaration<'a>),
    ConstructorDeclaration,
    DestructorDeclaration,
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
        let trait_use_clause = TraitUseClause::parser().map(Self::TraitUseClause);
        let method_declaration =
            MethodDeclaration::parser(statement_parser).map(Self::MethodDeclaration);

        choice((
            class_const_declaration,
            property_declaration,
            trait_use_clause,
            method_declaration,
        ))
        .labelled("PropertyDeclaration")
    }
}
