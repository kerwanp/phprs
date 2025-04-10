use chumsky::{input::ValueInput, prelude::*};
use phprs_lexer::Token;

use crate::parser::atoms::{
    name::variable_name::VariableName, r#type::type_declaration::TypeDeclaration,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariadicParameter<'a> {
    r#type: Option<TypeDeclaration<'a>>,
    reference: bool,
    name: VariableName<'a>,
}

impl<'a> VariadicParameter<'a> {
    pub fn parser<I>() -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let type_declaration = TypeDeclaration::parser().or_not();
        let reference = just(Token::Ampersand).or_not().map(|t| t.is_some());

        type_declaration
            .then(reference)
            .then_ignore(just(Token::DotDotDot))
            .then(VariableName::parser())
            .map(|((r#type, reference), name)| VariadicParameter {
                r#type,
                reference,
                name,
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{atoms::r#type::Type, tokenize};

    use super::*;

    fn parse(src: &str) -> Result<VariadicParameter, ()> {
        let token_stream = tokenize(src);

        VariadicParameter::parser()
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn simple() {
        let res = parse(r#"...$test"#);
        assert_eq!(
            res,
            Ok(VariadicParameter {
                r#type: None,
                reference: false,
                name: VariableName("$test")
            })
        );
    }

    #[test]
    fn reference() {
        let res = parse(r#"&...$test"#);
        assert_eq!(
            res,
            Ok(VariadicParameter {
                r#type: None,
                reference: true,
                name: VariableName("$test"),
            })
        );
    }

    #[test]
    fn typed() {
        let res = parse(r#"array ...$test"#);
        assert_eq!(
            res,
            Ok(VariadicParameter {
                r#type: Some(TypeDeclaration {
                    optional: false,
                    r#type: Type::Array
                }),
                reference: false,
                name: VariableName("$test"),
            })
        );
    }
}
