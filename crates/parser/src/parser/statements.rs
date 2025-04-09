pub mod class_declaration;
pub mod compound_statement;
pub mod const_declaration;
pub mod continue_statement;
pub mod declare_statement;
pub mod do_statement;
pub mod echo_statement;
pub mod expression_statement;
pub mod for_statement;
pub mod foreach_statement;
pub mod function_definition;
pub mod function_static_declaration;
pub mod global_declaration;
pub mod goto_statement;
pub mod if_statement;
pub mod interface_declaration;
pub mod named_label_statement;
pub mod namespace_definition;
pub mod namespace_use_declaration;
pub mod return_statement;
pub mod switch_statement;
pub mod throw_statement;
pub mod trait_declaration;
pub mod try_statement;
pub mod unset_statement;
pub mod while_statement;

use chumsky::prelude::*;
use chumsky::{error::Rich, extra, input::ValueInput, Parser};

use class_declaration::ClassDeclaration;
use compound_statement::CompoundStatement;
use const_declaration::ConstDeclaration;
use continue_statement::ContinueStatement;
use declare_statement::DeclareStatement;
use do_statement::DoStatement;
use echo_statement::EchoStatement;
use expression_statement::ExpressionStatement;
use for_statement::ForStatement;
use foreach_statement::ForeachStatement;
use function_definition::FunctionDefinition;
use function_static_declaration::FunctionStaticDeclaration;
use global_declaration::GlobalDeclaration;
use goto_statement::GotoStatement;
use if_statement::IfStatement;
use interface_declaration::InterfaceDeclaration;
use named_label_statement::NamedLabelStatement;
use namespace_definition::NamespaceDefinition;
use namespace_use_declaration::NamespaceUseDeclaration;
use phprs_lexer::Token;
use return_statement::ReturnStatement;
use switch_statement::SwitchStatement;
use throw_statement::ThrowStatement;
use trait_declaration::TraitDeclaration;
use try_statement::TryStatement;
use unset_statement::UnsetStatement;
use while_statement::WhileStatement;

use super::BoxedParser;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Statement<'a> {
    Compound(CompoundStatement<'a>),
    NamedLabel(NamedLabelStatement<'a>),
    Expression(ExpressionStatement<'a>),
    // SELECTION STATEMENTS
    If(Box<IfStatement<'a>>),
    Switch(SwitchStatement<'a>),
    // ITERATION STATEMENTS
    While(WhileStatement<'a>),
    Do(Box<DoStatement<'a>>),
    For(ForStatement<'a>),
    Foreach(ForeachStatement<'a>),
    // JUMP STATEMENTS
    Goto(GotoStatement<'a>),
    Continue(ContinueStatement<'a>),
    Return(ReturnStatement<'a>),
    Throw(ThrowStatement<'a>),
    Try(TryStatement<'a>),
    Declare(DeclareStatement<'a>),
    Echo(EchoStatement<'a>),
    Unset(UnsetStatement<'a>),
    ConstDeclaration(ConstDeclaration<'a>),
    FunctionDefinition(FunctionDefinition<'a>),
    ClassDeclaration(ClassDeclaration<'a>),
    InterfaceDeclaration(InterfaceDeclaration<'a>),
    TraitDeclaration(TraitDeclaration<'a>),
    NamespaceDefinition(NamespaceDefinition<'a>),
    NamespaceUseDeclaration(NamespaceUseDeclaration<'a>),
    GlobalDeclaration(GlobalDeclaration<'a>),
    FunctionStaticDeclaration(FunctionStaticDeclaration<'a>),
}

impl<'a> Statement<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        recursive(|parser| {
            let compound_statement =
                CompoundStatement::parser(parser.clone().boxed()).map(Self::Compound);
            let named_label_statement = NamedLabelStatement::parser().map(Self::NamedLabel);
            let expression_statement = ExpressionStatement::parser().map(Self::Expression);
            let echo_statement = EchoStatement::parser().map(Self::Echo);

            let namespace_use_declaration =
                NamespaceUseDeclaration::parser().map(Self::NamespaceUseDeclaration);

            // TODO: Handle if(): endif;
            let if_statement = IfStatement::parser(parser.clone().boxed())
                .map(|statement| Self::If(Box::new(statement)));
            // Ig parser chaining could be better here
            let switch_statement =
                SwitchStatement::parser(Self::list_parser(parser.clone().boxed()).boxed())
                    .map(Self::Switch);
            let while_statement = WhileStatement::parser(parser.clone().boxed()).map(Self::While);
            let do_statement = DoStatement::parser(parser.clone().boxed())
                .map(|statement| Self::Do(Box::new(statement)));
            let for_statement = ForStatement::parser(parser.clone().boxed()).map(Self::For);
            let goto_statement = GotoStatement::parser().map(Self::Goto);
            let continue_statement = ContinueStatement::parser().map(Self::Continue);
            let return_statement = ReturnStatement::parser().map(Self::Return);
            let throw_statement = ThrowStatement::parser().map(Self::Throw);
            let unset_statement = UnsetStatement::parser().map(Self::Unset);
            let namespace_definition =
                NamespaceDefinition::parser(parser.clone().boxed()).map(Self::NamespaceDefinition);
            let const_declaration = ConstDeclaration::parser().map(Self::ConstDeclaration);
            let global_declaration = GlobalDeclaration::parser().map(Self::GlobalDeclaration);
            let function_definition =
                FunctionDefinition::parser(parser.clone().boxed()).map(Self::FunctionDefinition);
            let function_static_declaration =
                FunctionStaticDeclaration::parser().map(Self::FunctionStaticDeclaration);
            let declare_statement =
                DeclareStatement::parser(parser.clone().boxed()).map(Self::Declare);
            let try_statement = TryStatement::parser(parser.clone().boxed()).map(Self::Try);
            let foreach_statement =
                ForeachStatement::parser(parser.clone().boxed()).map(Self::Foreach);
            let class_declaration =
                ClassDeclaration::parser(parser.clone().boxed()).map(Self::ClassDeclaration);
            let interface_declaration = InterfaceDeclaration::parser(parser.clone().boxed())
                .map(Self::InterfaceDeclaration);
            let trait_declaration =
                TraitDeclaration::parser(parser.clone().boxed()).map(Self::TraitDeclaration);

            choice((
                echo_statement,
                named_label_statement,
                expression_statement,
                namespace_use_declaration,
                compound_statement,
                if_statement,
                switch_statement,
                while_statement,
                do_statement,
                for_statement,
                goto_statement,
                continue_statement,
                return_statement,
                throw_statement,
                unset_statement,
                namespace_definition,
                const_declaration,
                global_declaration,
                function_definition,
                function_static_declaration,
                declare_statement,
                try_statement,
                foreach_statement,
                class_declaration,
                interface_declaration,
                trait_declaration,
            ))
            .boxed()
            .labelled("Statement")
        })
    }

    pub fn list_parser<I>(
        statement_parser: BoxedParser<'a, I, Statement<'a>>,
    ) -> impl Parser<'a, I, Vec<Self>, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        statement_parser.repeated().collect::<Vec<_>>()
    }
}
