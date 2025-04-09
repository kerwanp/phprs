use crate::parser::expressions::Expression;
use crate::parser::variables::new_variable::NewVariable;
use crate::parser::BoxedParser;
use chumsky::{input::ValueInput, span::SimpleSpan};
use chumsky::{prelude::*, Parser};

use phprs_lexer::Token;

use super::name::qualified_name::QualifiedName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ClassTypeDesignator<'a> {
    QualifiedName(QualifiedName<'a>),
    NewVariable(NewVariable<'a>),
}

impl<'a> ClassTypeDesignator<'a> {
    pub fn parser<I>(
        expression_parser: BoxedParser<'a, I, Expression<'a>>,
    ) -> impl Parser<'a, I, Self, extra::Err<Rich<'a, Token<'a>>>> + Clone
    where
        I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
    {
        let qualified_name = QualifiedName::parser().map(Self::QualifiedName);
        let new_variable = NewVariable::parser(expression_parser).map(Self::NewVariable);

        choice((new_variable, qualified_name))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenize;

    use super::*;

    fn parse(src: &str) -> Result<ClassTypeDesignator, ()> {
        let token_stream = tokenize(src);

        ClassTypeDesignator::parser(Expression::parser().boxed())
            .parse(token_stream)
            .into_result()
            .map_err(|_| ())
    }

    #[test]
    fn qualified() {
        let res = parse(r#"test"#);
        assert!(matches!(res, Ok(ClassTypeDesignator::QualifiedName(_))))
    }

    #[test]
    fn new() {
        let res = parse(r#"$test"#);
        assert!(matches!(res, Ok(ClassTypeDesignator::NewVariable(_))))
    }
}
