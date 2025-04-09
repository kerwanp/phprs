use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use crate::parser::statements::Statement;
use crate::parser::BoxedParser;
use phprs_lexer::Token;

use super::class_const_declaration::ClassConstDeclaration;
use super::enum_case_declaration::EnumCaseDeclaration;
use super::method_declaration::MethodDeclaration;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EnumMemberDeclaration<'a> {
    MethodDeclaration(MethodDeclaration<'a>),
    ClassConstDeclaration(ClassConstDeclaration<'a>),
    CaseDeclaration(EnumCaseDeclaration<'a>),
}

impl<'a> EnumMemberDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let method_declaration =
            MethodDeclaration::parser(statement_parser.clone()).map(Self::MethodDeclaration);
        let const_declaration = ClassConstDeclaration::parser(statement_parser.clone())
            .map(Self::ClassConstDeclaration);
        let case_declaration =
            EnumCaseDeclaration::parser(statement_parser).map(Self::CaseDeclaration);

        choice((method_declaration, const_declaration, case_declaration))
    }
}
