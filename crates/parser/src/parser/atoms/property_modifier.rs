use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use super::static_modifier::StaticModifier;
use super::visibility_modifier::VisibilityModifier;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PropertyModifier {
    Var,
    Visibility(VisibilityModifier),
    Static(StaticModifier),
    VisibilityAndStatic(VisibilityModifier, StaticModifier),
}

impl<'a> PropertyModifier {
    // TODO: Might be a better way to parse that
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let var = select! { Token::VarKeyword => Self::Var };

        let visiblity_first = VisibilityModifier::parser()
            .then(StaticModifier::parser().or_not())
            .map(|(visibility, r#static)| match r#static {
                Some(r#static) => Self::VisibilityAndStatic(visibility, r#static),
                None => Self::Visibility(visibility),
            });
        let static_first = StaticModifier::parser()
            .then(VisibilityModifier::parser().or_not())
            .map(|(r#static, visibility)| match visibility {
                Some(visibility) => Self::VisibilityAndStatic(visibility, r#static),
                None => Self::Static(r#static),
            });

        choice((var, visiblity_first, static_first))
    }
}
