use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use crate::parser::class::class_const_declaration::ClassConstDeclaration;
use crate::parser::class::method_declaration::MethodDeclaration;
use crate::parser::statements::Statement;
use crate::parser::BoxedParser;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InterfaceMemberDeclaration<'a> {
    ClassConstDeclaration(ClassConstDeclaration<'a>),
    MethodDeclaration(MethodDeclaration<'a>),
}

impl<'a> InterfaceMemberDeclaration<'a> {
    pub fn parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let class_const_declaration = ClassConstDeclaration::parser(statement_parser.clone())
            .map(Self::ClassConstDeclaration);
        let method_declaration =
            MethodDeclaration::parser(statement_parser).map(Self::MethodDeclaration);

        choice((class_const_declaration, method_declaration)).labelled("InterfaceMemberDeclaration")
    }
}
