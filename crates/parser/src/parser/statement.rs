use compound_statement::CompoundStatement;
use echo_statement::EchoStatement;
use expression_statement::ExpressionStatement;
use nom::{branch::alt, multi::many0, Parser};

use super::{util::ws, Error};

pub mod compound_statement;
pub mod echo_statement;
pub mod expression_statement;
pub mod named_label_statement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Statement<'a> {
    Compound(CompoundStatement<'a>),
    NamedLabel,
    Expression(ExpressionStatement<'a>),
    If,
    Switch,
    While,
    Do,
    For,
    Foreach,
    Goto,
    Continue,
    Break,
    Return,
    Throw,
    Try,
    Declare,
    Echo(EchoStatement<'a>),
    UnsetDeclaration,
    ConstDeclaration,
    FunctionDefinition,
    ClassDeclaration,
    InterfaceDeclaration,
    TraitDeclaration,
    NamespaceDefinition,
    NamespaceUseDeclaration,
    GlobalDeclaration,
    FunctionStaticDeclaration,
}

impl<'a> Statement<'a> {
    pub fn parse(input: &'a str) -> nom::IResult<&'a str, Self, Error<'a>> {
        let expresion = ExpressionStatement::parse.map(Self::Expression);
        let compound = CompoundStatement::parse.map(Self::Compound);
        let echo = EchoStatement::parse.map(Self::Echo);
        ws(alt((expresion, compound, echo))).parse(input)
    }

    pub fn parse_many(input: &'a str) -> nom::IResult<&'a str, Vec<Self>, Error<'a>> {
        many0(Self::parse).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn expression() {
        let variable = Statement::parse("0b11;");
        assert_matches!(variable, Ok(("", Statement::Expression(_))));
    }
}
