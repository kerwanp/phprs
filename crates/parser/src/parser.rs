pub mod expression;
pub mod literal;
pub mod name;
pub mod relative_scope;
pub mod scalar_type;
pub mod script;
pub mod statement;
pub mod util;
pub mod variable;

type Error<'a> = nom::error::VerboseError<&'a str>;
