use std::{fmt, ops::Range};

use logos::{Logos, Skip};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LexingError {
    #[default]
    Unknown,
}

#[derive(Logos, Clone, Debug, PartialEq, Eq, Hash)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f\r]+")]
pub enum Token<'a> {
    Unknown,
    #[token("<?php")]
    ScriptSectionStartTag,
    #[token("?>")]
    ScriptSectionEndTag,
    #[token("<?=")]
    ScriptSectionStartWithEchoTag,

    // TODO: Later we should add it
    #[regex(r#"\/\*([^*]|\*[^\/])*\*\/"#, |_| Skip)]
    #[regex(r#"#[^\r\n]*"#, |_| Skip)]
    #[regex(r#"//[^\r\n]*"#, |_| Skip)]
    Comment,

    #[regex("[a-zA-Z_]+")] // name-nondigit
    Name(&'a str),
    #[regex(r#"\$[a-zA-Z_][_a-zA-Z0-9]*"#)]
    VariableName(&'a str),

    // KEYWORDS
    #[token("abstract", ignore(case))]
    AbstractKeyword,
    #[token("and", ignore(case))]
    AndKeyword,
    #[token("array", ignore(case))]
    ArrayKeyword,
    #[token("as", ignore(case))]
    AsKeyword,
    #[token("break", ignore(case))]
    BreakKeyword,
    #[token("callable", ignore(case))]
    CallableKeyword,
    #[token("case", ignore(case))]
    CaseKeyword,
    #[token("catch", ignore(case))]
    CatchKeyword,
    #[token("class", ignore(case))]
    ClassKeyword,
    #[token("clone", ignore(case))]
    CloneKeyword,
    #[token("const", ignore(case))]
    ConstKeyword,
    #[token("continue", ignore(case))]
    ContinueKeyword,
    #[token("declare", ignore(case))]
    DeclareKeyword,
    #[token("default", ignore(case))]
    DefaultKeyword,
    #[token("die", ignore(case))]
    DieKeyword,
    #[token("do", ignore(case))]
    DoKeyword,
    #[token("echo", ignore(case))]
    EchoKeyword,
    #[token("else", ignore(case))]
    ElseKeyword,
    #[token("elseif", ignore(case))]
    ElseIfKeyword,
    #[token("empty", ignore(case))]
    EmptyKeyword,
    #[token("enddeclare", ignore(case))]
    EndDeclareKeyword,
    #[token("endfor", ignore(case))]
    EndForKeyword,
    #[token("endforeach", ignore(case))]
    EndForEachKeyword,
    #[token("endif", ignore(case))]
    EndIfKeyword,
    #[token("endswitch", ignore(case))]
    EndSwitchKeyword,
    #[token("endwhile", ignore(case))]
    EndWhileKeyword,
    #[token("eval", ignore(case))]
    EvalKeyword,
    #[token("exit", ignore(case))]
    ExitKeyword,
    #[token("extends", ignore(case))]
    ExtendsKeyword,
    #[token("final", ignore(case))]
    FinalKeyword,
    #[token("finally", ignore(case))]
    FinallyKeyword,
    #[token("for", ignore(case))]
    ForKeyword,
    #[token("foreach", ignore(case))]
    ForeachKeyword,
    #[token("function", ignore(case))]
    FunctionKeyword,
    #[token("global", ignore(case))]
    GlobalKeyword,
    #[token("goto", ignore(case))]
    GotoKeyword,
    #[token("if", ignore(case))]
    IfKeyword,
    #[token("implements", ignore(case))]
    ImplementsKeyword,
    #[token("include", ignore(case))]
    IncludeKeyword,
    #[token("include_once", ignore(case))]
    IncludeOnceKeyword,
    #[token("instance_of", ignore(case))]
    InstanceOfKeyword,
    #[token("instead_of", ignore(case))]
    InsteadOfKeyword,
    #[token("interface", ignore(case))]
    InterfaceKeyword,
    #[token("isset", ignore(case))]
    IssetKeyword,
    #[token("list", ignore(case))]
    ListKeyword,
    #[token("namespace", ignore(case))]
    NamespaceKeyword,
    #[token("new", ignore(case))]
    NewKeyword,
    #[token("or", ignore(case))]
    OrKeyword,
    #[token("print", ignore(case))]
    PrintKeyword,
    #[token("private", ignore(case))]
    PrivateKeyword,
    #[token("protected", ignore(case))]
    ProtectedKeyword,
    #[token("public", ignore(case))]
    PublicKeyword,
    #[token("require", ignore(case))]
    RequireKeyword,
    #[token("require_once", ignore(case))]
    RequireOnceKeyword,
    #[token("return", ignore(case))]
    ReturnKeyword,
    #[token("static", ignore(case))]
    StaticKeyword,
    #[token("switch", ignore(case))]
    SwitchKeyword,
    #[token("throw", ignore(case))]
    ThrowKeyword,
    #[token("trait", ignore(case))]
    TraitKeyword,
    #[token("try", ignore(case))]
    TryKeyword,
    #[token("unset", ignore(case))]
    UnsetKeyword,
    #[token("use", ignore(case))]
    UseKeyword,
    #[token("var", ignore(case))]
    VarKeyword,
    #[token("while", ignore(case))]
    WhileKeyword,
    #[token("xor", ignore(case))]
    XorKeyword,
    // WARNING: idk why but if yield is followed by 0x0a (LF) it does not match
    #[regex("yield[\r\n ]?", ignore(case))]
    YieldKeyword,
    #[regex(r#"yield[\t\n\f ]from"#, ignore(case))]
    YieldFromKeyword,
    #[token("fn", ignore(case))]
    FnKeyword,
    #[token("match", ignore(case))]
    MatchKeyword,
    #[token("enum", ignore(case))]
    EnumKeyword,
    #[token("readonly", ignore(case))]
    ReadonlyKeyword,
    // TODO: This should not be a keyword
    // #[token("halt")]
    // HaltCompilerKeyword,

    // TOKENS
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token(".")]
    Dot,
    #[token("->")]
    Arrow,
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,
    #[token("**")]
    AsteriskAsterisk,
    #[token("*")]
    Asterisk,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("~")]
    Tilde,
    #[token("!")]
    Exclamation,
    #[token("$")]
    Dollar,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("<<")]
    LessThanLessThan,
    #[token(">>")]
    GreaterThanGreaterThan,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreatherThan,
    #[token("<=")]
    LessThanEqual,
    #[token(">=")]
    GreaterThanEquals,
    #[token("==")]
    EqualsEquals,
    #[token("===")]
    EqualsEqualsEquals,
    #[token("!=")]
    ExclamationEquals,
    #[token("!==")]
    ExclamationEqualsEquals,
    #[token("^")]
    Caret,
    #[token("|")]
    Bar,
    #[token("&")]
    Ampersand,
    #[token("&&")]
    AmpersandAmpersand,
    #[token("||")]
    BarBar,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("=")]
    Equals,
    #[token("**=")]
    AsteriskAsteriskEquals,
    #[token("*=")]
    AsteriskEquals,
    #[token("/=")]
    SlashEquals,
    #[token("%=")]
    PercentEquals,
    #[token("+=")]
    PlusEquals,
    #[token("-=")]
    MinusEquals,
    #[token(".=")]
    DotEquals,
    #[token("<<=")]
    LessThanLessThanEquals,
    #[token(">>=")]
    GreaterThanGreaterThanEquals,
    #[token("&=")]
    AmpersandEquals,
    #[token("^=")]
    CaretEquals,
    #[token("|=")]
    BarEquals,
    #[token(",")]
    Comma,
    #[token("??")]
    QuestionQuestion,
    #[token("<=>")]
    LessThanEqualsGreaterThan,
    #[token("...")]
    DotDotDot,
    #[token("\\")]
    Backslack,
    #[token("::")]
    ColonColon,
    #[token("=>")]
    DoubleArrow,
    #[token("<>")]
    LessThanGreaterThan,
    #[token("@")]
    AtSymbol,
    #[token("`")]
    Backtick,
    #[token("?")]
    Question,
    #[token("??=")]
    QuestionQuestionEquals,
    #[token("?->")]
    QuestionArrow,
    // TODO: Idk
    // #[token("#")]
    // Attribute,
    #[token("'")]
    SingleQuote,
    #[token(r#"""#)]
    DoubleQuote,
    #[token("${")]
    DollarOpenBrace,
    #[token("{$")]
    OpenBraceDollar,

    // LITERALS
    // TODO: Add others
    #[regex(r#"[bB]?"(?:\\.|[^"\\])*""#)] // TODO: Improve this
    #[regex(r#"[bB]?'(?:\\.|[^'\\])*'"#)] // TODO: Improve this
    StringLiteral(&'a str),

    #[regex(r#"(0x)?[0-9]+"#, priority = 11)]
    IntegerLiteral(&'a str),

    // #[regex(r#"[.][0-9]+"#)]
    #[regex(r#"[0-9]*[.][0-9]+([eE][+-]?[0-9]+)?"#, priority = 12)]
    #[regex(r#"[0-9]+[.][0-9]*([eE][+-]?[0-9]+)?"#, priority = 13)]
    FloatingLiteral(&'a str),

    #[regex(r#"0[bB][01]+"#, priority = 7)]
    BinaryLiteral(&'a str),

    #[regex(r#"0[bB][0-9]+"#, priority = 6)]
    InvalidBinaryLiteral(&'a str),

    // RESERVED WORDS
    // TODO: Check if capitalized
    #[token("int")]
    IntReservedWord,
    #[token("float")]
    FloatReservedWord,
    #[token("true", ignore(case))]
    TrueReservedWord,
    #[token("false", ignore(case))]
    FalseReservedWord,
    #[token("string")]
    StringReservedWord,
    #[token("bool")]
    BoolReservedWord,
    #[token("null")]
    NullReservedWord,
    #[token("mixed")]
    MixedReservedWord,
    #[token("iterable")]
    IterableReservedWord,
    #[token("never")]
    NeverReservedWord,
    #[token("void")]
    VoidReservedWord,
    #[token("binary")]
    BinaryReservedWord,
    #[token("boolean")]
    BooleanReservedWord,
    #[token("double")]
    DoubleReservedWord,
    #[token("integer")]
    IntegerReservedWord,
    #[token("object")]
    ObjectReservedWord,
    #[token("real")]
    RealReservedWord,

    // TODO: What is this??
    ReturnType,
    ScriptSectionPrependText,

    // Manually added
    EndOfFile,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ScriptSectionStartTag => write!(f, "<?php"),
            Self::LessThan => write!(f, "<"),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub type Result<'a> = std::result::Result<Token<'a>, LexingError>;

pub fn lexer(content: &str) -> impl Iterator<Item = (Result, Range<usize>)> {
    Token::lexer(content)
        .spanned()
        .chain(Some((Ok(Token::EndOfFile), 0..0)))
}
