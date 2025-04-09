use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use super::class_modifier::ClassModifier;
use super::static_modifier::StaticModifier;
use super::visibility_modifier::VisibilityModifier;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MethodModifier {
    Visibility(VisibilityModifier),
    Static(StaticModifier),
    Class(ClassModifier),
}

impl<'a> MethodModifier {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        choice((
            VisibilityModifier::parser().map(Self::Visibility),
            StaticModifier::parser().map(Self::Static),
            ClassModifier::parser().map(Self::Class),
        ))
    }

    pub fn list_parser<I>() -> impl Parser<'a, I, Vec<Self>, extra::Err<Rich<'a, Token<'a>>>>
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        Self::parser().repeated().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<MethodModifier, ()> {
        let tokens = tokenize(src);

        MethodModifier::parser()
            .parse(tokens)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn public() {
        let res = parse(r#"public"#);
        assert_eq!(
            res,
            Ok(MethodModifier::Visibility(VisibilityModifier::Public))
        );
    }

    #[test]
    fn protected() {
        let res = parse(r#"static"#);
        assert_eq!(res, Ok(MethodModifier::Static(StaticModifier)));
    }

    #[test]
    fn private() {
        let res = parse(r#"abstract"#);
        assert_eq!(res, Ok(MethodModifier::Class(ClassModifier::Abstract)));
    }
}
